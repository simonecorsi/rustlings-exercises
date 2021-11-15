// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move(Point),
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move(Point { x: 10, y: 30 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
