// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

impl Message {

}

fn main() {
    println!("{:?}", Message::Quit(String::from("Quitting")));
    println!("{:?}", Message::Echo(String::from("wacha")));
    println!("{:?}", Message::Move(String::from("moving")));
    println!("{:?}", Message::ChangeColor(String::from("now its white")));
}
