//! 🚦 EN 1990 traffic groups rows mutation leaves: traffic group.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚦️insert-traffic-group/🦀️.rs"]
pub mod insert_traffic_group;
#[path = "../🚦️remove-traffic-group/🦀️.rs"]
pub mod remove_traffic_group;
//#endregion 🪢️Mounts
