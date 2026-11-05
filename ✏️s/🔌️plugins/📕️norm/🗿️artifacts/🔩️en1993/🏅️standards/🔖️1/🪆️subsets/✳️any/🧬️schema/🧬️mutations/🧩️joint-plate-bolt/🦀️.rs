//! 🧩️ EN 1993 joint plate mutation leaves: bolt class, bolt diameter, hole diameter and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧩️change-joint-plate-bolt-class/🦀️.rs"]
pub mod change_joint_plate_bolt_class;
#[path = "../🧩️change-joint-plate-bolt-diameter/🦀️.rs"]
pub mod change_joint_plate_bolt_diameter;
#[path = "../🧩️change-joint-plate-hole-diameter/🦀️.rs"]
pub mod change_joint_plate_hole_diameter;
#[path = "../🧩️change-joint-plate-bolt-rows/🦀️.rs"]
pub mod change_joint_plate_bolt_rows;
#[path = "../🧩️change-joint-plate-bolts-per-row/🦀️.rs"]
pub mod change_joint_plate_bolts_per_row;
#[path = "../🧩️change-joint-plate-pitch/🦀️.rs"]
pub mod change_joint_plate_pitch;
//#endregion 🪢️Mounts
