//! 📏️ EN 1996 wall geometry mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💧️change-exposure/🦀️.rs"]
pub mod change_exposure;
#[path = "../🧱change-support-sides/🦀️.rs"]
pub mod change_support_sides;
#[path = "../↕️change-wall-height/🦀️.rs"]
pub mod change_wall_height;
#[path = "../↔️change-wall-length/🦀️.rs"]
pub mod change_wall_length;
#[path = "../↕️change-wall-thickness/🦀️.rs"]
pub mod change_wall_thickness;
//#endregion 🪢️Mounts
