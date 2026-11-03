//! 🚆 EN 1991 rail load-model mutation leaves (EN 1991-2 §6).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚆change-rail-alpha/🦀️.rs"]
pub mod change_rail_alpha;
#[path = "../🚆change-assumed-rail-lm71/🦀️.rs"]
pub mod change_assumed_rail_lm71;
#[path = "../🚆change-assumed-rail-lm71-udl/🦀️.rs"]
pub mod change_assumed_rail_lm71_udl;
#[path = "../🚆change-assumed-rail-sw0/🦀️.rs"]
pub mod change_assumed_rail_sw0;
#[path = "../🚆change-assumed-rail-sw2/🦀️.rs"]
pub mod change_assumed_rail_sw2;
//#endregion 🪢️Mounts
