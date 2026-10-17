//! 🪵️ EN 1995 members mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌍️change-annex/🦀️.rs"]
pub mod change_annex;
#[path = "../🔥️change-member-fire-duration/🦀️.rs"]
pub mod change_member_fire_duration;
#[path = "../🏷️change-member-label-en/🦀️.rs"]
pub mod change_member_label_en;
#[path = "../🌧️change-member-service-class/🦀️.rs"]
pub mod change_member_service_class;
#[path = "../📍️change-member-support/🦀️.rs"]
pub mod change_member_support;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1995_mut_members_2::{change_member_label_de, change_member_role, change_member_strength_class, insert_member, remove_member};
