//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-friction-surfaces/🦀️.rs"]
pub mod change_joint_friction_surfaces;
#[path = "../🔩️change-joint-plate-fy/🦀️.rs"]
pub mod change_joint_plate_fy;
#[path = "../🔩️change-joint-plate-width/🦀️.rs"]
pub mod change_joint_plate_width;
#[path = "../🔩️change-joint-bolt-to-web/🦀️.rs"]
pub mod change_joint_bolt_to_web;
//#endregion 🪢️Mounts
