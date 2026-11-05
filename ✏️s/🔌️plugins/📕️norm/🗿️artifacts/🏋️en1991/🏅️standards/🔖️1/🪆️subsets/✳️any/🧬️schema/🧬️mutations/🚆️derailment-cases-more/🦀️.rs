//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚆️change-derailment-y/🦀️.rs"]
pub mod change_derailment_y;
#[path = "../🚆️change-derailment-assumed-force/🦀️.rs"]
pub mod change_derailment_assumed_force;
#[path = "../🚆️insert-derailment/🦀️.rs"]
pub mod insert_derailment;
#[path = "../🚆️remove-derailment/🦀️.rs"]
pub mod remove_derailment;
//#endregion 🪢️Mounts
