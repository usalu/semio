//! 🧪️ Kind fixture laws of `create-generation`, `delete-generation`, `rename-generation`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../➕create-generation/🧪️tests/🌱️appends/🦀️.rs"]
mod create_generation_appends;
#[path = "../../🗑️delete-generation/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_generation_removes;
#[path = "../../🗑️delete-generation/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_generation_removes_a_middle_row;
#[path = "../../🗑️delete-generation/🧪️tests/🔓️removes-an-unselected-middle-row/🦀️.rs"]
mod delete_generation_removes_an_unselected_middle_row;
#[path = "../../🏷️rename-generation/🧪️tests/🏷️retitles/🦀️.rs"]
mod rename_generation_retitles;
