//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬇️change-falling-y/🦀️.rs"]
pub mod change_falling_y;
#[path = "../⬇️change-falling-assumed-force/🦀️.rs"]
pub mod change_falling_assumed_force;
#[path = "../⬇️insert-falling/🦀️.rs"]
pub mod insert_falling;
#[path = "../⬇️remove-falling/🦀️.rs"]
pub mod remove_falling;
//#endregion 🪢️Mounts
