//! 🗼️ EN 1998 bridges-towers mutation leaves: bridge, tower.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-bridge-correlated-occupancy/🦀️.rs"]
pub mod change_bridge_correlated_occupancy;
#[path = "../✏️change-tower-correlated-occupancy/🦀️.rs"]
pub mod change_tower_correlated_occupancy;
//#endregion 🪢️Mounts
