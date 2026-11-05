//! 📍 EN 1992 geometry bar fields mutation leaves: bar point bend, bar point length.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️change-bar-point-length/🦀️.rs"]
pub mod change_bar_point_length;
#[path = "../📍️change-bar-point-bend/🦀️.rs"]
pub mod change_bar_point_bend;
//#endregion 🪢️Mounts
