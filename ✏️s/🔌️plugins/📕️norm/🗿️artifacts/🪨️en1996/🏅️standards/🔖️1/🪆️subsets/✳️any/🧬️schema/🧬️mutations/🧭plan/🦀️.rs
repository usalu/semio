//! 🧭 EN 1996 bracing layout and in-plane actions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧭change-plan-x/🦀️.rs"]
pub mod change_plan_x;
#[path = "../🧭change-plan-y/🦀️.rs"]
pub mod change_plan_y;
#[path = "../📐️change-plan-angle/🦀️.rs"]
pub mod change_plan_angle;
#[path = "../↔️change-in-plane-shear/🦀️.rs"]
pub mod change_in_plane_shear;
#[path = "../🔄change-in-plane-moment/🦀️.rs"]
pub mod change_in_plane_moment;
//#endregion 🪢️Mounts
