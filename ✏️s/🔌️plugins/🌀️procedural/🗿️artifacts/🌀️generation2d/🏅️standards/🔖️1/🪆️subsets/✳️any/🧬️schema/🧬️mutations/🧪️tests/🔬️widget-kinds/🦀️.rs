//! 🧪️ Kind fixture laws of `create-widget`, `delete-widget`, `replace-widget`, `move-widget`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🗑️delete-widget/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_widget_removes_note_a_and_flags_the_dangling_synapse;
#[path = "../../🗑️delete-widget/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_widget_removes_a_middle_row;
#[path = "../../📍️move/🧪️tests/📍️repositions/🦀️.rs"]
mod move_repositions_note_a_on_the_canvas;
#[path = "../../🔁️replace-widget/🧪️tests/✍️rewrites/🦀️.rs"]
mod replace_widget_rewrites_the_note_b_body_in_place;
#[path = "../../🌱️create-widget/🧪️tests/📝️inserts/🦀️.rs"]
mod create_widget_inserts_note_c_at_index_2;
