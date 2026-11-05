//! 🧩️ EN 1993 joint plate mutation leaves: joint id, label, role and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧩️change-joint-plate-joint-id/🦀️.rs"]
pub mod change_joint_plate_joint_id;
#[path = "../🧩️change-joint-plate-label/🦀️.rs"]
pub mod change_joint_plate_label;
#[path = "../🧩️change-joint-plate-role/🦀️.rs"]
pub mod change_joint_plate_role;
#[path = "../🧩️change-joint-plate-plate-height/🦀️.rs"]
pub mod change_joint_plate_plate_height;
#[path = "../🧩️change-joint-plate-plate-width/🦀️.rs"]
pub mod change_joint_plate_plate_width;
#[path = "../🧩️change-joint-plate-plate-thickness/🦀️.rs"]
pub mod change_joint_plate_plate_thickness;
#[path = "../🧩️change-joint-plate-plate-count/🦀️.rs"]
pub mod change_joint_plate_plate_count;
#[path = "../🧩️change-joint-plate-plate-fy/🦀️.rs"]
pub mod change_joint_plate_plate_fy;
#[path = "../🧩️change-joint-plate-plate-fu/🦀️.rs"]
pub mod change_joint_plate_plate_fu;
//#endregion 🪢️Mounts
