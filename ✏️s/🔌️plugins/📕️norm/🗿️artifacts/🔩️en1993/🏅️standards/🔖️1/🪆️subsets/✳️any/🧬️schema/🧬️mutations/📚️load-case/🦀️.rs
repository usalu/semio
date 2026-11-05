//! 📚️ EN 1993 load case mutation leaves: name, kind, category.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📚️change-load-case-name/🦀️.rs"]
pub mod change_load_case_name;
#[path = "../📚️change-load-case-kind/🦀️.rs"]
pub mod change_load_case_kind;
#[path = "../📚️change-load-case-category/🦀️.rs"]
pub mod change_load_case_category;
//#endregion 🪢️Mounts
