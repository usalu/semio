//! 🪚️ EN 1996 chases mutation leaves: chase, chase kind, chase wall and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪚️change-chase-wall/🦀️.rs"]
pub mod change_chase_wall;
#[path = "../🪚️change-chase-kind/🦀️.rs"]
pub mod change_chase_kind;
#[path = "../🪚️insert-chase/🦀️.rs"]
pub mod insert_chase;
#[path = "../🪚️remove-chase/🦀️.rs"]
pub mod remove_chase;
//#endregion 🪢️Mounts
