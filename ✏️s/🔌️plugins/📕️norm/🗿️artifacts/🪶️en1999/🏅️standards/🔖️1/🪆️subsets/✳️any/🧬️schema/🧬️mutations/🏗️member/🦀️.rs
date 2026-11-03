//! 🏗️ EN 1999 member mutation leaves: buckling length y, buckling length t.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-buckling-length-y/🦀️.rs"]
pub mod change_member_buckling_length_y;
#[path = "../🏗️change-member-buckling-length-t/🦀️.rs"]
pub mod change_member_buckling_length_t;
#[path = "../🏗️change-member-section-id/🦀️.rs"]
pub mod change_member_section_id;
#[path = "../🏗️change-member-material-id/🦀️.rs"]
pub mod change_member_material_id;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1999_mut_member_more::{change_member_length, change_member_support, change_member_ltb_length, change_member_c1, change_member_restrained_ltb};
