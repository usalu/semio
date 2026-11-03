//! 🧱️ EN 1995 entity-rows mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-taper/🦀️.rs"]
pub mod insert_taper;
#[path = "../➖️remove-taper/🦀️.rs"]
pub mod remove_taper;
#[path = "../➕️insert-cutout/🦀️.rs"]
pub mod insert_cutout;
#[path = "../➖️remove-cutout/🦀️.rs"]
pub mod remove_cutout;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_entity_rows_more::{insert_fastener, remove_fastener,insert_panel, remove_panel, insert_floor, remove_floor};
