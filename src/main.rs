use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request : {:#?}", http_request);
    let mut request_header = http_request[0].split(" ");

    if request_header.next() == Some("GET") {
        let status_line: &str;
        let file_name = request_header.next().unwrap();
        let content_type: &str;
        let path = format!(".{file_name}");
        let contents = match fs::read(&path) {
            Ok(string) => {
                status_line = "HTTP/1.1 200 OK";
                content_type = get_content_type(std::path::Path::new(&path));
                string
            },
            Err(_e) => {
                status_line = "HTTP/1.1 404 NOT FOUND";
                content_type = "text/html; charset=UTF-8";
                get_file_not_found_html().to_string().as_bytes().to_vec()
            },
        };

        let length = contents.len();

        //let response = format!("{status_line}\r\nContent-Type: {content_type} Content-Length: {length}\r\n\r\n{contents}");
        //stream.write_all(response.as_bytes()).unwrap();
        let response = format!("{status_line}\r\nContent-Type: {content_type}\nContent-Length: {length}\r\n\r\n");
        let response = response.as_bytes();
        let mut response = response.to_vec();
        for byte in contents {
            response.push(byte);
        }
        

        let _ = stream.write_all(response.as_slice());
    }
}

fn get_file_not_found_html() -> &'static str{
    return "<h1>Error 404 : file not found</h1>";
}

fn get_content_type(path: &std::path::Path) -> &str {
    let extension: &str = match path.extension(){
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };
    return match extension {
        "html" => "text/html;charset=UTF-8",
        "js" => "text/javascript;charset=UTF-8",
        "css" => "text/css;charset=UTF-8",
        "wasm" => "application/wasm",
        "ico" => "image/png",
        "png" => "image/png",
        "jpg"|"jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "",
    }
}