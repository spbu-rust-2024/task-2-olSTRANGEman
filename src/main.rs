use std::io;



fn check(line: &str) -> String{
    let line_b = line.as_bytes();
    let mut rem = &line[0..1];

    
        
    for i in 0..line_b.len()-1{
        for j in 1..=i{
            if line_b[i+j] != line_b[i-j]{
                // не смог объединить два условия для окончания 
                if (i+j)-(i-j)-1 > rem.len(){
                    rem = &line[i-j+1..i+j];
                }
                    break;
            }
            if i+j == line_b.len()-1{
                if (i+j)-(i-j)-1 > rem.len(){
                    rem = &line[i-j..i+j+1];
                }
                    break;
            }
        }
    }


        for i in 0..line_b.len()-1{
            for j in 1..=i{
                if line_b[i+j-1] != line_b[i-j]{
                    if (i+j-1)-(i-j)-1 > rem.len(){
                        rem = &line[i-j+1..i+j-1];
                    }
                        break;
                }
                if i+j-1 == line_b.len()-1{
                    if (i+j-1)-(i-j)-1 > rem.len(){
                    rem = &line[i-j..i+j];
                }
                    break;
            }
        }
    }
    
    rem.to_string()
    }

//проблема с проверкой последнего числа, разница &str\&String\str\String



fn main(){
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input is not a string");
    let input = input.trim();
    println!("{}",check(&input));
}
