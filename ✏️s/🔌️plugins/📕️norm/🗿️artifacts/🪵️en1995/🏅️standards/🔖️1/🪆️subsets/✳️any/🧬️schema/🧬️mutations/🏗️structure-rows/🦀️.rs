//! 🏗️ EN 1995 structure-rows mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-diaphragm/🦀️.rs"]
pub mod insert_diaphragm;
#[path = "../➖️remove-diaphragm/🦀️.rs"]
pub mod remove_diaphragm;
#[path = "../➕️insert-truss/🦀️.rs"]
pub mod insert_truss;
#[path = "../➖️remove-truss/🦀️.rs"]
pub mod remove_truss;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_structure_rows_more::{insert_bracing, remove_bracing, insert_deck, remove_deck};
