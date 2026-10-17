//! 🪜️ EN 1995 members mutation leaves (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏷️change-member-label-de/🦀️.rs"]
pub mod change_member_label_de;
#[path = "../🎯️change-member-role/🦀️.rs"]
pub mod change_member_role;
#[path = "../🛡️change-member-strength-class/🦀️.rs"]
pub mod change_member_strength_class;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
//#endregion 🪢️Mounts
