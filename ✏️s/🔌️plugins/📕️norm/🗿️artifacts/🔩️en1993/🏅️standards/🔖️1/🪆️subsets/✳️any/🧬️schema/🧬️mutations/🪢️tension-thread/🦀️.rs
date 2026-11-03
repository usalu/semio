//! 🪢️ EN 1993 tension component mutation leaves: thread factor, rod stress area, rod fu.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪢️change-tension-thread-factor/🦀️.rs"]
pub mod change_tension_thread_factor;
#[path = "../🪢️change-tension-rod-stress-area/🦀️.rs"]
pub mod change_tension_rod_stress_area;
#[path = "../🪢️change-tension-rod-fu/🦀️.rs"]
pub mod change_tension_rod_fu;
//#endregion 🪢️Mounts
