//! 🪜️ EN 1991 traffic mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚛change-assumed-bridge-lm2/🦀️.rs"]
pub mod change_assumed_bridge_lm2;
#[path = "../👥change-assumed-bridge-lm4/🦀️.rs"]
pub mod change_assumed_bridge_lm4;
#[path = "../🛣️change-assumed-bridge-udl/🦀️.rs"]
pub mod change_assumed_bridge_udl;
#[path = "../↔️change-bridge-lane-width/🦀️.rs"]
pub mod change_bridge_lane_width;
#[path = "../🏗️change-bridge-span/🦀️.rs"]
pub mod change_bridge_span;
//#endregion 🪢️Mounts
