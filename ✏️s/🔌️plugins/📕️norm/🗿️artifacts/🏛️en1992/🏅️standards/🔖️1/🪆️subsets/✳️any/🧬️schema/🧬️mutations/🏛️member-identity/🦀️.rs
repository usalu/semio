//! 🏛️ EN 1992 member identity mutation leaves: member concrete grade id, member deflection sensitive, member kind and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️change-member-label-en/🦀️.rs"]
pub mod change_member_label_en;
#[path = "../🏛️change-member-label-de/🦀️.rs"]
pub mod change_member_label_de;
#[path = "../🏛️change-member-kind/🦀️.rs"]
pub mod change_member_kind;
#[path = "../🏛️change-member-concrete-grade-id/🦀️.rs"]
pub mod change_member_concrete_grade_id;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1992_mut_member_identity_more::{change_member_rebar_grade_id, change_member_support, change_member_use_fem, change_member_deflection_sensitive};
