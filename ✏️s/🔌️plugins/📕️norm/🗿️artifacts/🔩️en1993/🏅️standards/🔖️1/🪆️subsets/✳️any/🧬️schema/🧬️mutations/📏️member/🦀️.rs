//! 📏️ EN 1993 member mutation leaves: label, member type, section id and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-member-label/🦀️.rs"]
pub mod change_member_label;
#[path = "../📏️change-member-member-type/🦀️.rs"]
pub mod change_member_member_type;
#[path = "../📏️change-member-section-id/🦀️.rs"]
pub mod change_member_section_id;
#[path = "../📏️change-member-material-id/🦀️.rs"]
pub mod change_member_material_id;
#[path = "../📏️change-member-length/🦀️.rs"]
pub mod change_member_length;
#[path = "../📏️change-member-buckling-length-y/🦀️.rs"]
pub mod change_member_buckling_length_y;
#[path = "../📏️change-member-buckling-length-z/🦀️.rs"]
pub mod change_member_buckling_length_z;
//#endregion 🪢️Mounts
