//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-bridge-category/🦀️.rs"]
pub mod change_bridge_category;
#[path = "../🌉️change-bridge-method/🦀️.rs"]
pub mod change_bridge_method;
#[path = "../🌉️change-bridge-consequence/🦀️.rs"]
pub mod change_bridge_consequence;
#[path = "../🌉️change-bridge-heavy-vehicle-weight/🦀️.rs"]
pub mod change_bridge_heavy_vehicle_weight;
#[path = "../🌉️change-bridge-reference-weight/🦀️.rs"]
pub mod change_bridge_reference_weight;
//#endregion 🪢️Mounts
