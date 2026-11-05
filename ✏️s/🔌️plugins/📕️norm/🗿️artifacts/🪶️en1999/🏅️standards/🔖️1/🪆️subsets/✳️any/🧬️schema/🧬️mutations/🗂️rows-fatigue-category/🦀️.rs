//! 🗂️ EN 1999 fatigue detail site mutation leaves: insert fatigue category, remove fatigue category.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-fatigue-category/🦀️.rs"]
pub mod insert_fatigue_category;
#[path = "../➖remove-fatigue-category/🦀️.rs"]
pub mod remove_fatigue_category;
//#endregion 🪢️Mounts
