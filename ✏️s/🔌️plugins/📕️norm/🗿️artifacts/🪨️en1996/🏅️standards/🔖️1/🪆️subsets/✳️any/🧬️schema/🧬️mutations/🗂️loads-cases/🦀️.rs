//! 🗂️ EN 1996 load-case mutation leaves: design situations and load-case rows.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎭️change-design-situation/🦀️.rs"]
pub mod change_design_situation;
#[path = "../🎭️change-load-case-situation/🦀️.rs"]
pub mod change_load_case_situation;
#[path = "../➕️insert-load-case/🦀️.rs"]
pub mod insert_load_case;
#[path = "../➖️remove-load-case/🦀️.rs"]
pub mod remove_load_case;
//#endregion 🪢️Mounts
