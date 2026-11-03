//! 🛣️ EN 1991 bridge lanes mutation leaves: bridge lanes, lane lm1 lane, lane offset and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛣️change-lane-width/🦀️.rs"]
pub mod change_lane_width;
#[path = "../🛣️change-lane-offset/🦀️.rs"]
pub mod change_lane_offset;
#[path = "../🛣️change-lane-lm1-lane/🦀️.rs"]
pub mod change_lane_lm1_lane;
#[path = "../🛣️insert-bridge-lane/🦀️.rs"]
pub mod insert_bridge_lane;
#[path = "../🛣️remove-bridge-lane/🦀️.rs"]
pub mod remove_bridge_lane;
//#endregion 🪢️Mounts
