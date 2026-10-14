//! 🏗️ EN 1991 cranes mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../↔️change-assumed-crane-horizontal/🦀️.rs"]
pub mod change_assumed_crane_horizontal;
#[path = "../➖change-assumed-crane-wheel/🦀️.rs"]
pub mod change_assumed_crane_wheel;
#[path = "../🏗️change-crane-claimed/🦀️.rs"]
pub mod change_crane_claimed;
#[path = "../💥change-crane-class/🦀️.rs"]
pub mod change_crane_class;
#[path = "../➕change-hoist-class/🦀️.rs"]
pub mod change_hoist_class;
#[path = "../⏫change-hoisting-speed/🦀️.rs"]
pub mod change_hoisting_speed;
//#endregion 🪢️Mounts
