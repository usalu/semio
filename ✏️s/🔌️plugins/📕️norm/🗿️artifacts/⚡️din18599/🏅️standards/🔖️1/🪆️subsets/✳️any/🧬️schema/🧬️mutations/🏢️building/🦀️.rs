//! 🏢️ DIN V 18599 building mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱change-attachment/🦀️.rs"]
pub mod change_attachment;
#[path = "../🎛️change-automation-class/🦀️.rs"]
pub mod change_automation_class;
#[path = "../🏠️change-building-category/🦀️.rs"]
pub mod change_building_category;
#[path = "../🌉change-delta-u-wb/🦀️.rs"]
pub mod change_delta_u_wb;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_din18599_mut_building_more::{change_geg_qp_factor, change_heated_volume_m3, change_method, change_net_floor_area_m2, change_use_class};
