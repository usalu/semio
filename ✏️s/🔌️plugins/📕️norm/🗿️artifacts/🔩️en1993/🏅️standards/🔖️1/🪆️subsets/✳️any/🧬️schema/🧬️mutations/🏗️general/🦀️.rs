//! 🏗️ EN 1993 general mutation leaves.

//#region 🪢️Mounts
#![allow(async_fn_in_trait)]
#[path = "../🌍️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../🎯change-reliability-class/🦀️.rs"]
pub mod change_reliability_class;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
//#endregion 🪢️Mounts


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_norm_en1993_mut_general_materials::{insert_material, insert_section, remove_material, remove_section};
