//! 🚆 RailDerailment mutation leaves: insert, remove and one set-field kind per settable field.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⚙️insert-rail-derailment/🦀️.rs"]
pub mod insert_rail_derailment;
#[path = "../⚙️remove-rail-derailment/🦀️.rs"]
pub mod remove_rail_derailment;
#[path = "../⚙️change-rail-derailment-offset/🦀️.rs"]
pub mod change_rail_derailment_offset;
#[path = "../⚙️change-rail-derailment-loaded-length/🦀️.rs"]
pub mod change_rail_derailment_loaded_length;
#[path = "../⚙️change-rail-derailment-assumed-line-load/🦀️.rs"]
pub mod change_rail_derailment_assumed_line_load;
#[path = "../🛤️change-rail-derailment-situation/🦀️.rs"]
pub mod change_rail_derailment_situation;
//#endregion 🪢️Mounts
