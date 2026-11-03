//! 🗂️ EN 1999 scenario mutation leaves: insert and remove fire scenario, fatigue detail and cold-formed sheet.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-fire-scenario/🦀️.rs"]
pub mod insert_fire_scenario;
#[path = "../➖remove-fire-scenario/🦀️.rs"]
pub mod remove_fire_scenario;
#[path = "../➕insert-fatigue-detail/🦀️.rs"]
pub mod insert_fatigue_detail;
#[path = "../➖remove-fatigue-detail/🦀️.rs"]
pub mod remove_fatigue_detail;
#[path = "../➕insert-cold-formed-sheet/🦀️.rs"]
pub mod insert_cold_formed_sheet;
#[path = "../➖remove-cold-formed-sheet/🦀️.rs"]
pub mod remove_cold_formed_sheet;
//#endregion 🪢️Mounts
