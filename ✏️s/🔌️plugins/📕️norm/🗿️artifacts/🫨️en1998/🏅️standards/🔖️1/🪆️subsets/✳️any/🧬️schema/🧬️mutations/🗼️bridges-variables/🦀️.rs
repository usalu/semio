//! 🗼️ EN 1998 bridges variables mutation leaves: category, qk n, insert.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-bridge-variable-category/🦀️.rs"]
pub mod change_bridge_variable_category;
#[path = "../✏️change-bridge-variable-qk-n/🦀️.rs"]
pub mod change_bridge_variable_qk_n;
#[path = "../➕️insert-bridge-variable/🦀️.rs"]
pub mod insert_bridge_variable;
#[path = "../➖️remove-bridge-variable/🦀️.rs"]
pub mod remove_bridge_variable;
//#endregion 🪢️Mounts
