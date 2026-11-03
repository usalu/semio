//! 🧱️ EN 1993 plated panel mutation leaves: stiffener shape, stiffener depth, stiffener thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️change-plated-stiffener-shape/🦀️.rs"]
pub mod change_plated_stiffener_shape;
#[path = "../🧱️change-plated-stiffener-depth/🦀️.rs"]
pub mod change_plated_stiffener_depth;
#[path = "../🧱️change-plated-stiffener-thickness/🦀️.rs"]
pub mod change_plated_stiffener_thickness;
#[path = "../🧱️change-plated-stiff-flange-width/🦀️.rs"]
pub mod change_plated_stiff_flange_width;
#[path = "../🧱️change-plated-stiffener-flange-thk/🦀️.rs"]
pub mod change_plated_stiffener_flange_thk;
#[path = "../🧱️change-plated-lateral-pressure/🦀️.rs"]
pub mod change_plated_lateral_pressure;
#[path = "../🧱️change-plated-lat-pressure-coef/🦀️.rs"]
pub mod change_plated_lat_pressure_coef;
//#endregion 🪢️Mounts
