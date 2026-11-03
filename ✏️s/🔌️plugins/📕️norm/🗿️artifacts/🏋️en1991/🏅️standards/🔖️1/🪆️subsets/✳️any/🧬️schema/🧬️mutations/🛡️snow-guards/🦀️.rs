//! 🛡️ SnowGuard mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️change-snow-guard-assumed-force/🦀️.rs"]
pub mod change_snow_guard_assumed_force;
#[path = "../⚙️change-snow-guard-slope-length/🦀️.rs"]
pub mod change_snow_guard_slope_length;
#[path = "../⚙️insert-snow-guard/🦀️.rs"]
pub mod insert_snow_guard;
#[path = "../⚙️remove-snow-guard/🦀️.rs"]
pub mod remove_snow_guard;
#[path = "../🛤️change-snow-guard-roof-id/🦀️.rs"]
pub mod change_snow_guard_roof_id;
//#endregion 🪢️Mounts
