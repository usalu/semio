//! 🧩️ EN 1993 joint plate mutation leaves: gauge, end distance, edge distance and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧩️change-joint-plate-gauge/🦀️.rs"]
pub mod change_joint_plate_gauge;
#[path = "../🧩️change-joint-plate-end-distance/🦀️.rs"]
pub mod change_joint_plate_end_distance;
#[path = "../🧩️change-joint-plate-edge-distance/🦀️.rs"]
pub mod change_joint_plate_edge_distance;
#[path = "../🧩️change-joint-plate-shear-planes/🦀️.rs"]
pub mod change_joint_plate_shear_planes;
#[path = "../🧩️change-joint-plate-eccentricity/🦀️.rs"]
pub mod change_joint_plate_eccentricity;
#[path = "../🧩️change-joint-plate-weld-throat/🦀️.rs"]
pub mod change_joint_plate_weld_throat;
#[path = "../🧩️change-joint-plate-weld-fu/🦀️.rs"]
pub mod change_joint_plate_weld_fu;
#[path = "../🧩️change-joint-plate-weld-beta-w/🦀️.rs"]
pub mod change_joint_plate_weld_beta_w;
//#endregion 🪢️Mounts
