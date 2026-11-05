//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-row-bolt-to-web/🦀️.rs"]
pub mod change_row_bolt_to_web;
#[path = "../🔩️change-row-edge-distance/🦀️.rs"]
pub mod change_row_edge_distance;
#[path = "../🔩️change-row-extension-to-flange/🦀️.rs"]
pub mod change_row_extension_to_flange;
#[path = "../🔩️change-row-extension-edge/🦀️.rs"]
pub mod change_row_extension_edge;
//#endregion 🪢️Mounts
