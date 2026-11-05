//! 🧵 EN 1996 movement joints mutation leaves: movement joint, movement joint position, movement joint wall and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧵️change-movement-joint-wall/🦀️.rs"]
pub mod change_movement_joint_wall;
#[path = "../🧵️change-movement-joint-position/🦀️.rs"]
pub mod change_movement_joint_position;
#[path = "../🧵️insert-movement-joint/🦀️.rs"]
pub mod insert_movement_joint;
#[path = "../🧵️remove-movement-joint/🦀️.rs"]
pub mod remove_movement_joint;
//#endregion 🪢️Mounts
