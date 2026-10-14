//! 📏️ EN 1992 members mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛡️change-member-cover/🦀️.rs"]
pub mod change_member_cover;
#[path = "../📐️change-member-effective-depth/🦀️.rs"]
pub mod change_member_effective_depth;
#[path = "../🌦️change-member-exposure/🦀️.rs"]
pub mod change_member_exposure;
#[path = "../↕️change-member-height/🦀️.rs"]
pub mod change_member_height;
#[path = "../🌉️change-member-span/🦀️.rs"]
pub mod change_member_span;
#[path = "../↔️change-member-width/🦀️.rs"]
pub mod change_member_width;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
#[path = "../🔀️reorder-members/🦀️.rs"]
pub mod reorder_members;
//#endregion 🪢️Mounts
