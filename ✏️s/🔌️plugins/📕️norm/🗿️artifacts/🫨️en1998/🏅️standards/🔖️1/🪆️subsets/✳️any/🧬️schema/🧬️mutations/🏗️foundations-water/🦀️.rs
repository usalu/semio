//! 🏗️ EN 1998 foundations water mutation leaves: water height m, saturated gamma, permeability m s.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-retaining-wall-water-height-m/🦀️.rs"]
pub mod change_retaining_wall_water_height_m;
#[path = "../✏️change-retaining-wall-saturated-gamma/🦀️.rs"]
pub mod change_retaining_wall_saturated_gamma;
#[path = "../✏️change-retaining-wall-permeability-ms/🦀️.rs"]
pub mod change_retaining_wall_permeability_m_s;
//#endregion 🪢️Mounts
