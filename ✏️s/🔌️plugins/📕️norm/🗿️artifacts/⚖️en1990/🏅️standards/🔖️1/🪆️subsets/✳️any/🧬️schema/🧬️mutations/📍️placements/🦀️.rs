//! 📍 EN 1990 placements mutation leaves: placement x, placement y, placement yaw and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📍️change-placement-x/🦀️.rs"]
pub mod change_placement_x;
#[path = "../📍️change-placement-y/🦀️.rs"]
pub mod change_placement_y;
#[path = "../📍️change-placement-z/🦀️.rs"]
pub mod change_placement_z;
#[path = "../📍️change-placement-yaw/🦀️.rs"]
pub mod change_placement_yaw;
//#endregion 🪢️Mounts
