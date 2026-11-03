//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💧️change-water-flow-length/🦀️.rs"]
pub mod change_water_flow_length;
#[path = "../💧️change-water-well-radius/🦀️.rs"]
pub mod change_water_well_radius;
#[path = "../💧️change-water-well-water-depth/🦀️.rs"]
pub mod change_water_well_water_depth;
#[path = "../💧️change-water-influence-radius/🦀️.rs"]
pub mod change_water_influence_radius;
#[path = "../💧️change-water-well-capacity/🦀️.rs"]
pub mod change_water_well_capacity;
//#endregion 🪢️Mounts
