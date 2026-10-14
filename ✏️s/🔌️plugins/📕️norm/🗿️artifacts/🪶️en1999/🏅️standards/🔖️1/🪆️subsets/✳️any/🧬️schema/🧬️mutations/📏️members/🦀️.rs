//! 📏️ EN 1999 members mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕add-member/🦀️.rs"]
pub mod add_member;
#[path = "../🔩change-bolt-count/🦀️.rs"]
pub mod change_bolt_count;
#[path = "../⚗️change-material-designation/🦀️.rs"]
pub mod change_material_designation;
#[path = "../📏️change-member-buckling-length/🦀️.rs"]
pub mod change_member_buckling_length;
#[path = "../⤴️change-member-my-ed/🦀️.rs"]
pub mod change_member_m_y_ed;
#[path = "../🏋️change-member-n-ed/🦀️.rs"]
pub mod change_member_n_ed;
#[path = "../🧱change-plate-thickness/🦀️.rs"]
pub mod change_plate_thickness;
#[path = "../🔥️change-weld-throat/🦀️.rs"]
pub mod change_weld_throat;
#[path = "../➖remove-member/🦀️.rs"]
pub mod remove_member;
//#endregion 🪢️Mounts
