//! 🗂️ EN 1999 collection mutation leaves: insert and remove material, section, connection and shell.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕insert-material/🦀️.rs"]
pub mod insert_material;
#[path = "../➖remove-material/🦀️.rs"]
pub mod remove_material;
#[path = "../➕insert-section/🦀️.rs"]
pub mod insert_section;
#[path = "../➖remove-section/🦀️.rs"]
pub mod remove_section;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1999_mut_collection_rows_connection_shell::{insert_connection, remove_connection, insert_shell, remove_shell};
