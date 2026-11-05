//! 🛤 EN 1991 rail dynamics mutation leaves: rail dynamic, rail dynamic determinant length, rail dynamic maintenance and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛤️change-rail-dynamic-determinant-length/🦀️.rs"]
pub mod change_rail_dynamic_determinant_length;
#[path = "../🛤️change-rail-dynamic-maintenance/🦀️.rs"]
pub mod change_rail_dynamic_maintenance;
#[path = "../🛤️insert-rail-dynamic/🦀️.rs"]
pub mod insert_rail_dynamic;
#[path = "../🛤️remove-rail-dynamic/🦀️.rs"]
pub mod remove_rail_dynamic;
//#endregion 🪢️Mounts
