//! 🪜️ EN 1991 traffic load-model mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
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
#[path = "../📍change-bridge-tandem-x/🦀️.rs"]
pub mod change_bridge_tandem_x;
#[path = "../🌀change-bridge-curve-radius/🦀️.rs"]
pub mod change_bridge_curve_radius;
//#endregion 🪢️Mounts
