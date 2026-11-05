//! 🧱 EN 1997 structure mutation leaves (uplift cases, ground anchors, embedded walls).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-anchor/🦀️.rs"]
pub mod insert_anchor;
#[path = "../➕insert-embedded-wall/🦀️.rs"]
pub mod insert_embedded_wall;
#[path = "../➕insert-uplift-case/🦀️.rs"]
pub mod insert_uplift_case;
#[path = "../➖remove-anchor/🦀️.rs"]
pub mod remove_anchor;
#[path = "../➖remove-embedded-wall/🦀️.rs"]
pub mod remove_embedded_wall;
#[path = "../➖remove-uplift-case/🦀️.rs"]
pub mod remove_uplift_case;
//#endregion 🪢️Mounts
