//! 📏️ EN 1999 members mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩change-bolt-count/🦀️.rs"]
pub mod change_bolt_count;
#[path = "../⚗️change-material-designation/🦀️.rs"]
pub mod change_material_designation;
#[path = "../📏️change-member-buckling-length/🦀️.rs"]
pub mod change_member_buckling_length;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1999_mut_members_more::{change_member_m_y_ed, change_member_n_ed, change_plate_thickness, change_weld_throat, remove_member, insert_member};
