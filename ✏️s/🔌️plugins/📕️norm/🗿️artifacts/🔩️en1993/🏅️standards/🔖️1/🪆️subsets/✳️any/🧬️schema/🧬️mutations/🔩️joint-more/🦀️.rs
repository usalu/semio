//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-joint-bolt-rows/🦀️.rs"]
pub mod change_joint_bolt_rows;
#[path = "../🔩️change-joint-bolts-per-row/🦀️.rs"]
pub mod change_joint_bolts_per_row;
#[path = "../🔩️change-joint-pitch/🦀️.rs"]
pub mod change_joint_pitch;
#[path = "../🔩️change-joint-gauge/🦀️.rs"]
pub mod change_joint_gauge;
#[path = "../🔩️change-joint-end-distance/🦀️.rs"]
pub mod change_joint_end_distance;
//#endregion 🪢️Mounts
