use core::str;
//use crate::player::brain::*;
//use crate::player::*;
pub mod connect4;
pub mod nim12;

use crate::game::connect4::Connect4;
use crate::game::nim12::Nim12;
use log::trace;
use std::fmt;

#[derive(Clone, Default)]
pub struct GameStatic {
    pub name: String,
    pub players: Vec<String>,
    state_size: usize, // number of bits
                        //  hashmap: HashMap<String, usize>,
                        //  all_legal_moves: Vec<String>,
                        //  initial_state: Vec<bool>,
}

impl GameStatic {
    pub fn new(
        name: String,
        players: Vec<String>,
        state_size: usize,
        //hashmap: HashMap<String, usize>,
        //all_legal_moves: Vec<String>,
        //initial_state: Vec<bool>,
    ) -> GameStatic {
        Self {
            name,
            players,
            state_size,
            //    hashmap: hashmap,
            //    all_legal_moves: all_legal_moves,
            //    initial_state,
        }
    }

    pub fn get_state_size(&self) -> usize {
        self.state_size
    }
}

pub trait Playable {
    //type GameStaticType: GameStatic; // Add this line

    // fn new() -> Self;
    fn reset(&mut self);
    fn get_game_static(&self) -> GameStatic; // Update this line
    fn get_valid_moves(&self) -> Vec<String>;
    fn is_terminal(&self) -> bool;

    fn get_winner(&self) -> Option<String>;
    fn get_active_player(&self) -> String;
    fn play(&mut self, play_move: &str);
    fn get_bit_state(&self, perspective: &str) -> Vec<bool>;
    fn get_bit_state_from_bit_state_and_move(
        &self,
        perspective: &str,
        tmp_state: &[bool],
        cmove: &str,
    ) -> Vec<bool>;
    fn get_valid_moves_from_bit_state(
        &self,
        perspective: &str,
        tmp_state: &[bool],
    ) -> Vec<String>;
    fn pretty_print(&self);
    // fn get_bit_state_from_state(tmp_state: &Vec<bool>)->Vec<bool>;
    // fn get_state_from_bit_state()->HashMap<String, String>
}

// Define an enum that contains different game types, each variant implementing Playable.
pub enum Game<'a> {
    Connect4(&'a mut Connect4), // Assuming Connect4 is a game type you've defined
    Nim12(&'a mut Nim12), //      Nim12(nim12::Nim12), // Assuming nim12 is a game from an external library
}
// Implement the Playable trait for the Game enum.
impl Playable for Game<'_> {
    // type Player = game::Player; // Specify the player type, which may need to be adjusted based on your code
    fn reset(&mut self) {
        match self {
            Game::Connect4(game) => game.reset(),
            Game::Nim12(game) => game.reset(),
        }
    }
    fn get_game_static(&self) -> GameStatic {
        match self {
            Game::Connect4(game) => game.get_game_static(),
            Game::Nim12(game) => game.get_game_static(),
        }
    }
    fn get_valid_moves(&self) -> Vec<String> {
        match self {
            Game::Connect4(game) => game.get_valid_moves(),
            Game::Nim12(game) => game.get_valid_moves(),
        }
    }
    fn is_terminal(&self) -> bool {
        match self {
            Game::Connect4(game) => game.is_terminal(),
            Game::Nim12(game) => game.is_terminal(),
        }
    }
    fn get_winner(&self) -> Option<String> {
        match self {
            Game::Connect4(game) => game.get_winner(),
            Game::Nim12(game) => game.get_winner(),
        }
    }
    fn get_active_player(&self) -> String {
        match self {
            Game::Connect4(game) => game.get_active_player(),
            Game::Nim12(game) => game.get_active_player(),
        }
    }
    fn play(&mut self, play_move: &str) {
        match self {
            Game::Connect4(game) => game.play(play_move),
            Game::Nim12(game) => game.play(play_move),
        }
    }
    fn get_bit_state(&self, perspective: &str) -> Vec<bool> {
        match self {
            Game::Connect4(game) => game.get_bit_state(perspective),
            Game::Nim12(game) => game.get_bit_state(perspective),
        }
    }
    fn get_bit_state_from_bit_state_and_move(
        &self,
        perspective: &str,
        tmp_state: &[bool],
        cmove: &str,
    ) -> Vec<bool> {
        match self {
            Game::Connect4(game) => {
                game.get_bit_state_from_bit_state_and_move(perspective, tmp_state, cmove)
            }
            Game::Nim12(game) => {
                game.get_bit_state_from_bit_state_and_move(perspective, tmp_state, cmove)
            }
        }
    }

    fn get_valid_moves_from_bit_state(
        &self,
        perspective: &str,
        tmp_state: &[bool],
    ) -> Vec<String> {
        match self {
            Game::Connect4(game) => game.get_valid_moves_from_bit_state(perspective, tmp_state),
            Game::Nim12(game) => game.get_valid_moves_from_bit_state(perspective, tmp_state),
        }
    }

    fn pretty_print(&self) {
        match self {
            Game::Connect4(game) => game.pretty_print(),
            Game::Nim12(game) => game.pretty_print(),
        }
    }
}

impl std::fmt::Display for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*            */
        match self {
            Game::Connect4(_game) => {
                write!(f, "Connect4 .....")?;
            },
            x => {
                write!(f, "Unimplemented Display {}", x)?;
            }
        }
        write!(f, "Game")
    }
}


pub enum TerminalState {
    Me,
    Opponent,
    Draw,
    //Cancelled,
}

impl std::fmt::Debug for TerminalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalState::Me => write!(f, "Me"),
            TerminalState::Opponent => write!(f, "Oponent"),
            TerminalState::Draw => write!(f, "Draw"),
            // TerminalState::Cancelled => write!(f, "Cancelled"),
        }
    }
}

// Get the terminal state from a bit vector
/// # Arguments
///
/// * `state` - The state of the game as a bit vector.
pub fn get_terminal_state_from_bit_state(state: &Vec<bool>) -> Option<TerminalState> {
    // Draw = state[4]
    // Me = state[5]
    //Opponent = state[6]

    if state[4] {
        //draw
        trace!("get_terminal_state_from_bit_state {:?}", state);
        return Some(TerminalState::Draw);
    } else if state[5] {
        //me
        return Some(TerminalState::Me);
    } else if state[6] {
        //opponent
        trace!("get_terminal_state_from_bit_state {:?}", state);

        return Some(TerminalState::Opponent);
    }
    None
}

pub fn get_active_player_from_bit_state(game_static: &GameStatic,state: &Vec<bool>) -> String{
    if state[2] ^ state[3]{
        game_static.players[0].clone()
    } else {
        game_static.players[1].clone()
    }
}
