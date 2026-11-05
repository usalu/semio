//! 📍️ EN 1993 row mutation leaves: placement.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-placement/🦀️.rs"]
pub mod insert_placement;
#[path = "../➖️remove-placement/🦀️.rs"]
pub mod remove_placement;
//#endregion 🪢️Mounts
