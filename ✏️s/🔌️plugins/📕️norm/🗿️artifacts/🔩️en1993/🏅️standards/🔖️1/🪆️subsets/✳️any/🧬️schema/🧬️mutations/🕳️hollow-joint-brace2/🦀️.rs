//! 🕳️ EN 1993 hollow joint mutation leaves: brace2 width, brace2 height, brace2 thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-hollow-brace2-width/🦀️.rs"]
pub mod change_hollow_brace2_width;
#[path = "../🕳️change-hollow-brace2-height/🦀️.rs"]
pub mod change_hollow_brace2_height;
#[path = "../🕳️change-hollow-brace2-thickness/🦀️.rs"]
pub mod change_hollow_brace2_thickness;
#[path = "../🕳️change-hollow-brace2-angle/🦀️.rs"]
pub mod change_hollow_brace2_angle;
#[path = "../🕳️change-hollow-brace2-force/🦀️.rs"]
pub mod change_hollow_brace2_force;
#[path = "../🕳️change-hollow-gap/🦀️.rs"]
pub mod change_hollow_gap;
#[path = "../🕳️change-hollow-brace-fy/🦀️.rs"]
pub mod change_hollow_brace_fy;
//#endregion 🪢️Mounts
