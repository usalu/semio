//! 🏗️ EN 1993 general mutation leaves.

//#region 🪢️Mounts
#[path = "../🌍️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../➕️insert-material/🦀️.rs"]
pub mod insert_material;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➕️insert-section/🦀️.rs"]
pub mod insert_section;
#[path = "../➖️remove-material/🦀️.rs"]
pub mod remove_material;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
#[path = "../➖️remove-section/🦀️.rs"]
pub mod remove_section;
#[path = "../📊️update-member-properties/🦀️.rs"]
pub mod update_member_properties;
#[path = "../✨️update-stainless-inputs/🦀️.rs"]
pub mod update_stainless_inputs;
#[path = "../↕️update-through-thickness-inputs/🦀️.rs"]
pub mod update_through_thickness_inputs;
//#endregion 🪢️Mounts

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
