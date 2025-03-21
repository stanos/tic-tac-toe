use std::io;

use console::Term;


fn draw_play_field(play_field: &Vec<Vec<&str>>, turn_count: &mut i32, term: &Term) {
    term.write_line("   1   2   3").unwrap();
    term.write_line(&format!("1  {} | {} | {}", play_field[0][0], play_field[0][1], play_field[0][2])).unwrap();
    term.write_line(" -----------").unwrap();
    term.write_line(&format!("2  {} | {} | {}", play_field[1][0], play_field[1][1], play_field[1][2])).unwrap();
    term.write_line(" -----------").unwrap();
    term.write_line(&format!("3  {} | {} | {}", play_field[2][0], play_field[2][1], play_field[2][2])).unwrap();
    term.write_line("").unwrap();
    *turn_count += 1;
}

fn prompt_user() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    return input;
}

fn main(){
    let mut current_player = "X";
    let mut play_field = vec![vec![" "; 3]; 3];
    let mut turn_count = 0;
    let term = Term::stdout();
    draw_play_field(&play_field, &mut turn_count, &term);
    loop {
        if turn_count == 9 {
            term.write_line("Game over! It's a draw.").unwrap();
            break;
        }
        
        term.write_line(&format!("Player {}, enter the row and column number where you want to place your mark (e.g. 1 2). Enter 'exit' to quit the game.", current_player)).unwrap();
        let input = prompt_user();
        let input = input.trim();
        let input: Vec<&str> = input.split(" ").collect();
        if input == ["exit"] {
            break;
        }
        
        if input.len() != 2 {
            term.write_line("Invalid input. Please enter two numbers separated by a space.").unwrap();
            continue;
        }

        let row = match input[0].parse::<usize>() {
            Ok(num) if num > 0 && num <= 3 => num - 1,
            _ => {
                term.write_line("Invalid row number. Please enter a number between 1 and 3.").unwrap();
                continue;
            }
        };
        
        let col = match input[1].parse::<usize>() {
            Ok(num) if num > 0 && num <= 3 => num - 1,
            _ => {
                term.write_line("Invalid column number. Please enter a number between 1 and 3.").unwrap();
                continue;
            }
        };
        
        if play_field[row][col] == " " {
            play_field[row][col] = current_player;
            if current_player == "X" {
                current_player = "O";
            } else {
                current_player = "X";
            }
            if col == 0 {
                if play_field[row][1] == play_field[row][2] && play_field[row][1] == play_field[row][0] {
                    term.write_line(&format!("Player {} wins!", play_field[row][0])).unwrap();
                    break;
                }
            } else if col == 1 {
                if play_field[row][0] == play_field[row][2] && play_field[row][0] == play_field[row][1] {
                    term.write_line(&format!("Player {} wins!", play_field[0][col])).unwrap();
                    break;
                }
            } else {
                if play_field[row][0] == play_field[row][1] && play_field[row][0] == play_field[row][2] {
                    term.write_line(&format!("Player {} wins!", play_field[0][col])).unwrap();
                    break;
                }
            }
            if row == 0 {
                if play_field[1][col] == play_field[2][col] && play_field[1][col] == play_field[0][col] {
                    term.write_line(&format!("Player {} wins!", play_field[0][col])).unwrap();
                    break;
                }
            } else if row == 1 {
                if play_field[0][col] == play_field[2][col] && play_field[0][col] == play_field[1][col] {
                    term.write_line(&format!("Player {} wins!", play_field[0][col])).unwrap();
                    break;
                }
            } else {
                if play_field[0][col] == play_field[1][col] && play_field[0][col] == play_field[2][col] {
                    term.write_line(&format!("Player {} wins!", play_field[0][col])).unwrap();
                    break;
                }
            }
        } else {
            turn_count -= 1;
            term.write_line("Cell is already occupied. Try again.").unwrap();
        }
        draw_play_field(&play_field, &mut turn_count, &term);
    }
}