//! 🕸️ EN 1993 row mutation leaves: frame-node, frame-bar.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-frame-node/🦀️.rs"]
pub mod insert_frame_node;
#[path = "../➖️remove-frame-node/🦀️.rs"]
pub mod remove_frame_node;
#[path = "../➕️insert-frame-bar/🦀️.rs"]
pub mod insert_frame_bar;
#[path = "../➖️remove-frame-bar/🦀️.rs"]
pub mod remove_frame_bar;
//#endregion 🪢️Mounts
