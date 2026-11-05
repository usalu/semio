//! 🕳️ EN 1993 hollow joint mutation leaves: brace1 width, brace1 height, brace1 thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-hollow-brace1-width/🦀️.rs"]
pub mod change_hollow_brace1_width;
#[path = "../🕳️change-hollow-brace1-height/🦀️.rs"]
pub mod change_hollow_brace1_height;
#[path = "../🕳️change-hollow-brace1-thickness/🦀️.rs"]
pub mod change_hollow_brace1_thickness;
#[path = "../🕳️change-hollow-brace1-angle/🦀️.rs"]
pub mod change_hollow_brace1_angle;
#[path = "../🕳️change-hollow-brace1-force/🦀️.rs"]
pub mod change_hollow_brace1_force;
//#endregion 🪢️Mounts
