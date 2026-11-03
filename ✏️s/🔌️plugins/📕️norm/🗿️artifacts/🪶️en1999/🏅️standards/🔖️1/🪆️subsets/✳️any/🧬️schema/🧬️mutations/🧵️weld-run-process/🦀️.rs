//! 🧵️ EN 1999 weld run mutation leaves: t junction, direction, haz extent.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧵️change-weld-run-t-junction/🦀️.rs"]
pub mod change_weld_run_t_junction;
#[path = "../🧵️change-weld-run-direction/🦀️.rs"]
pub mod change_weld_run_direction;
#[path = "../🧵️change-weld-run-haz-extent/🦀️.rs"]
pub mod change_weld_run_haz_extent;
//#endregion 🪢️Mounts
