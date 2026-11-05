//! 🧱️ EN 1994 slab action rows mutation leaves: slab action.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️insert-slab-action/🦀️.rs"]
pub mod insert_slab_action;
#[path = "../🧱️remove-slab-action/🦀️.rs"]
pub mod remove_slab_action;
//#endregion 🪢️Mounts
