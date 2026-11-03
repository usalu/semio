//! 🪢️ EN 1993 tension component mutation leaves: pin diameter, pin plate thickness, pin gap and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪢️change-tension-pin-diameter/🦀️.rs"]
pub mod change_tension_pin_diameter;
#[path = "../🪢️change-tension-pin-plate-thickness/🦀️.rs"]
pub mod change_tension_pin_plate_thickness;
#[path = "../🪢️change-tension-pin-gap/🦀️.rs"]
pub mod change_tension_pin_gap;
#[path = "../🪢️change-tension-pin-outer-thickness/🦀️.rs"]
pub mod change_tension_pin_outer_thickness;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_tension_pin_more::{change_tension_pin_inner_thickness, change_tension_pin_fup, change_tension_pin_fyp, change_tension_pin_plate_fy};
