use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

    println!("Enter the degree");
    stdin.read_line(&mut buffer).expect("Invalid read line");
    let degree = buffer.trim().parse::<u32>().ok().expect("Wanted a number");
    for i in 0..(2 as u32).pow(degree) {
        let base = format!("{number:>0width$b}", number = i, width = degree as usize).replace("0", "X").replace("1", "Y");

        println!("{}  {}", i, base);
        // let mut chars = vec!();
        // for c in base.chars() {
        //     chars.push(c);
        // }
        // let mut s: String = String::new();
        // let mut i = 0;
        // while i < chars.len() {
        //     let mut power: u8 = 0;
        //     for j in i..chars.len() {
        //         if chars[i] == chars[j] {
        //             power += 1;
        //         } else {
        //             break;
        //         }
        //     }
        //     s.push(chars[i]);
        //     for c in format!("{}", power).chars() {
        //         s.push(c);
        //     }
        //     s.push(',');
        //     s.push(' ');
        //     i += power as usize;
        // }
        // println!("{}", s.split_at(s.len() - 2).0);
    }
}
