fn main() {
    for i in 1..101 {
        if 1 % 3 == 0 && 1 % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

