//! 🏢️ EN 1998 buildings variables mutation leaves: category, qk n, insert.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-storey-variable-category/🦀️.rs"]
pub mod change_storey_variable_category;
#[path = "../✏️change-storey-variable-qk-n/🦀️.rs"]
pub mod change_storey_variable_qk_n;
#[path = "../➕️insert-storey-variable/🦀️.rs"]
pub mod insert_storey_variable;
#[path = "../➖️remove-storey-variable/🦀️.rs"]
pub mod remove_storey_variable;
//#endregion 🪢️Mounts
