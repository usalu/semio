//! 🧪️ Kind fixture laws of `create-widget`, `update-widget`, `delete-widget`, `move-widget`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🌱️create-widget/🧪️tests/📝️inserts/🦀️.rs"]
mod create_widget_inserts;
#[path = "../../🩹update-widget/🧪️tests/🎚️retunes/🦀️.rs"]
mod update_widget_retunes;
#[path = "../../❌delete-widget/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_widget_removes;
#[path = "../../❌delete-widget/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_widget_removes_a_middle_row;
#[path = "../../📍️move-widget/🧪️tests/📍️repositions/🦀️.rs"]
mod move_widget_repositions;
