//! 🧱️ EN 1991 compartment mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🗺️change-fire-compartment-area/🦀️.rs"]
pub mod change_fire_compartment_area;
#[path = "../📐change-fire-compartment-height/🦀️.rs"]
pub mod change_fire_compartment_height;
#[path = "../🪟change-fire-opening-factor/🦀️.rs"]
pub mod change_fire_opening_factor;
#[path = "../🧱change-fire-thermal-inertia/🦀️.rs"]
pub mod change_fire_thermal_inertia;
//#endregion 🪢️Mounts
