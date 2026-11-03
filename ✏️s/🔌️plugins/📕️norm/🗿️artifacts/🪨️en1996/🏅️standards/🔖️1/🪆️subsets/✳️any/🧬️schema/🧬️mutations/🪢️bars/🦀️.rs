//! 🪢️ EN 1996 reinforcing bars mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪢️change-bar-kind/🦀️.rs"]
pub mod change_bar_kind;
#[path = "../🪢️change-bar-diameter/🦀️.rs"]
pub mod change_bar_diameter;
#[path = "../🪢️change-bar-length/🦀️.rs"]
pub mod change_bar_length;
#[path = "../🪢️change-bar-spacing/🦀️.rs"]
pub mod change_bar_spacing;
#[path = "../➕️insert-bar/🦀️.rs"]
pub mod insert_bar;
#[path = "../➖️remove-bar/🦀️.rs"]
pub mod remove_bar;
//#endregion 🪢️Mounts
