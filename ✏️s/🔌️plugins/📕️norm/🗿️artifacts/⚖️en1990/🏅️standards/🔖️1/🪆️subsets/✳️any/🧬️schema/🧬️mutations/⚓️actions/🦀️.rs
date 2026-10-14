//! ⚓️ EN 1990 actions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚓️change-permanents/🦀️.rs"]
pub mod change_permanents;
#[path = "../🏋️change-variables/🦀️.rs"]
pub mod change_variables;
#[path = "../➕insert-permanent/🦀️.rs"]
pub mod insert_permanent;
#[path = "../📥insert-variable/🦀️.rs"]
pub mod insert_variable;
#[path = "../➖remove-permanent/🦀️.rs"]
pub mod remove_permanent;
#[path = "../📤remove-variable/🦀️.rs"]
pub mod remove_variable;
//#endregion 🪢️Mounts
