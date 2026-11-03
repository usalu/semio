//! 🔩️ EN 1993 joint mutation leaves: edge distance, shear planes, plate thickness and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-edge-distance/🦀️.rs"]
pub mod change_joint_edge_distance;
#[path = "../🔩️change-joint-shear-planes/🦀️.rs"]
pub mod change_joint_shear_planes;
#[path = "../🔩️change-joint-plate-thickness/🦀️.rs"]
pub mod change_joint_plate_thickness;
#[path = "../🔩️change-joint-plate-fu/🦀️.rs"]
pub mod change_joint_plate_fu;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1993_mut_joint_edge_more::{change_joint_weld_throat, change_joint_weld_length, change_joint_weld_fu, change_joint_weld_grade};
