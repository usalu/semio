//! 🧪️ Kind fixture laws of `create-generation`, `delete-generation`, `select-generation`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../➕create-generation/🧪️tests/🌱️appends/🦀️.rs"]
mod create_generation_appends_generation_2_and_selects_it;
#[path = "../../➖delete-generation/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_generation_removes_the_selected_generation_2_and_falls_back_to_generation_1;
#[path = "../../➖delete-generation/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_generation_removes_a_middle_row;
#[path = "../../➖delete-generation/🧪️tests/🔓️unselected-middle/🦀️.rs"]
mod delete_generation_removes_an_unselected_middle_row;
#[path = "../../👆️select-generation/🧪️tests/👆️picks/🦀️.rs"]
mod select_generation_picks;
