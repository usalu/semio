//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-wall-base-width/🦀️.rs"]
pub mod change_wall_base_width;
#[path = "../➕insert-footing/🦀️.rs"]
pub mod insert_footing;
#[path = "../📥insert-pile/🦀️.rs"]
pub mod insert_pile;
#[path = "../➖remove-footing/🦀️.rs"]
pub mod remove_footing;
#[path = "../📤remove-pile/🦀️.rs"]
pub mod remove_pile;
//#endregion 🪢️Mounts
