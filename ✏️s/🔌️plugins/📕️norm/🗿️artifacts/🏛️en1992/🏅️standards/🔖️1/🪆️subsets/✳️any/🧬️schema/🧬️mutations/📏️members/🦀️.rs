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
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1992_mut_members_more::{change_member_span, change_member_width, insert_member, remove_member, change_member_order};
