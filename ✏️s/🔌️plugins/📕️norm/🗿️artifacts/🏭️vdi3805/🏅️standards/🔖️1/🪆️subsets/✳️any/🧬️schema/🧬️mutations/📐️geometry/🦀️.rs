//! 📐️ VDI 3805 geometry mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️change-curve-points/🦀️.rs"]
pub mod change_curve_points;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_vdi3805_mut_geometry_more::{change_geometry_parameters, remove_curve, remove_geometry, remove_geometry_connection, resize_geometry, add_curve, add_geometry, add_geometry_connection};
