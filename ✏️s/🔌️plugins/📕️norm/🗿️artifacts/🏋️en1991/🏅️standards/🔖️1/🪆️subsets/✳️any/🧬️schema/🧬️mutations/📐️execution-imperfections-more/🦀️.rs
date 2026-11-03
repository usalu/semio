//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📐️change-imperfection-y/🦀️.rs"]
pub mod change_imperfection_y;
#[path = "../📐️change-imperfection-assumed-force/🦀️.rs"]
pub mod change_imperfection_assumed_force;
#[path = "../📐️insert-imperfection/🦀️.rs"]
pub mod insert_imperfection;
#[path = "../📐️remove-imperfection/🦀️.rs"]
pub mod remove_imperfection;
//#endregion 🪢️Mounts
