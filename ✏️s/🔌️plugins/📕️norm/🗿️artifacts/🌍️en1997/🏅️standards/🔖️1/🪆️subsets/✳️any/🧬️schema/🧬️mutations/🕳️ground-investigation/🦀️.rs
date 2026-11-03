//! 🕳️ EN 1997 ground investigation mutation leaves: position x, position y, ground level and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕳️change-ground-investigation-position-x/🦀️.rs"]
pub mod change_ground_investigation_position_x;
#[path = "../🕳️change-ground-investigation-position-y/🦀️.rs"]
pub mod change_ground_investigation_position_y;
#[path = "../🕳️change-ground-investigation-level/🦀️.rs"]
pub mod change_ground_investigation_level;
#[path = "../🕳️change-ground-investigation-depth/🦀️.rs"]
pub mod change_ground_investigation_depth;
#[path = "../🕳️change-ground-investigation-kind/🦀️.rs"]
pub mod change_ground_investigation_kind;
//#endregion 🪢️Mounts
