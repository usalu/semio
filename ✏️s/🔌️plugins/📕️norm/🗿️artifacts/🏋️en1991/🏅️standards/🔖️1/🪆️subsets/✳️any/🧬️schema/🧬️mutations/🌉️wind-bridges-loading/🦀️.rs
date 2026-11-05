//! 🌉 EN 1991 wind bridges loading mutation leaves: wind bridge assumed force, wind bridge ground height, wind bridge span and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-wind-bridge-traffic-kind/🦀️.rs"]
pub mod change_wind_bridge_traffic_kind;
#[path = "../🌉️change-wind-bridge-ground-height/🦀️.rs"]
pub mod change_wind_bridge_ground_height;
#[path = "../🌉️change-wind-bridge-span/🦀️.rs"]
pub mod change_wind_bridge_span;
#[path = "../🌉️change-wind-bridge-assumed-force/🦀️.rs"]
pub mod change_wind_bridge_assumed_force;
//#endregion 🪢️Mounts
