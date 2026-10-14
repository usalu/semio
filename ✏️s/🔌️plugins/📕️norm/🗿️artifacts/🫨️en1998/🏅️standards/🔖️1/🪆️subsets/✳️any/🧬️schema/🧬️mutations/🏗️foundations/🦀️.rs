//! 🏗️ EN 1998 foundations mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪨insert-foundation/🦀️.rs"]
pub mod insert_foundation;
#[path = "../🧱️insert-retaining-wall/🦀️.rs"]
pub mod insert_retaining_wall;
#[path = "../➖️remove-foundation/🦀️.rs"]
pub mod remove_foundation;
#[path = "../➖️remove-retaining-wall/🦀️.rs"]
pub mod remove_retaining_wall;
//#endregion 🪢️Mounts
