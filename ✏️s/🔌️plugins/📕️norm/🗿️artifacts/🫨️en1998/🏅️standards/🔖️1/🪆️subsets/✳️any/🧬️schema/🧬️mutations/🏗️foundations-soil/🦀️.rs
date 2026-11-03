//! 🏗️ EN 1998 foundations soil mutation leaves: foundation id, thickness m, unit weight n m3.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-soil-layer-foundation-id/🦀️.rs"]
pub mod change_soil_layer_foundation_id;
#[path = "../✏️change-soil-layer-thickness-m/🦀️.rs"]
pub mod change_soil_layer_thickness_m;
#[path = "../✏️change-soil-unit-wt-nm3/🦀️.rs"]
pub mod change_soil_unit_wt_n_m3;
#[path = "../✏️change-soil-layer-vs-ms/🦀️.rs"]
pub mod change_soil_layer_vs_m_s;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_foundations_soil_more::{change_soil_layer_n1_60, change_soil_layer_fines_percent, insert_soil_layer, remove_soil_layer};
