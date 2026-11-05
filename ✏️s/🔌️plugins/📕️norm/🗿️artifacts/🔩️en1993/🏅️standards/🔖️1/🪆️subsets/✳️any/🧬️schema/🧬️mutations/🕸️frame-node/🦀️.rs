//! 🕸️ EN 1993 frame node mutation leaves: label, x, z and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕸️change-frame-node-label/🦀️.rs"]
pub mod change_frame_node_label;
#[path = "../🕸️change-frame-node-x/🦀️.rs"]
pub mod change_frame_node_x;
#[path = "../🕸️change-frame-node-z/🦀️.rs"]
pub mod change_frame_node_z;
#[path = "../🕸️change-frame-node-support/🦀️.rs"]
pub mod change_frame_node_support;
#[path = "../🕸️change-frame-node-load-x/🦀️.rs"]
pub mod change_frame_node_load_x;
#[path = "../🕸️change-frame-node-load-z/🦀️.rs"]
pub mod change_frame_node_load_z;
//#endregion 🪢️Mounts
