//! 💧 EN 1997 layer water mutation leaves: layer id, piezometric depth, permeability and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💧️change-water-layer-id/🦀️.rs"]
pub mod change_water_layer_id;
#[path = "../💧️change-water-piezometric-depth/🦀️.rs"]
pub mod change_water_piezometric_depth;
#[path = "../💧️change-water-permeability/🦀️.rs"]
pub mod change_water_permeability;
#[path = "../💧️change-water-head-difference/🦀️.rs"]
pub mod change_water_head_difference;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1997_mut_layer_water_more::{change_water_flow_length, change_water_well_radius, change_water_well_water_depth, change_water_influence_radius, change_water_well_capacity};
