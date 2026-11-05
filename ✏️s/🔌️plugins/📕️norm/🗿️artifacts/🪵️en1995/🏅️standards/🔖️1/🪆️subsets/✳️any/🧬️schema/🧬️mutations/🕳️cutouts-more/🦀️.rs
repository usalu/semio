//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-cutout-depth/🦀️.rs"]
pub mod change_cutout_depth;
#[path = "../🕳️change-cutout-centre-height/🦀️.rs"]
pub mod change_cutout_centre_height;
#[path = "../🕳️change-cutout-slope/🦀️.rs"]
pub mod change_cutout_slope;
#[path = "../🕳️change-cutout-rods/🦀️.rs"]
pub mod change_cutout_rods;
#[path = "../🕳️change-cutout-rod-resistance/🦀️.rs"]
pub mod change_cutout_rod_resistance;
//#endregion 🪢️Mounts
