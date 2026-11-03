//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-building-t1-method/🦀️.rs"]
pub mod change_building_t1_method;
#[path = "../✏️change-building-t1-given-s/🦀️.rs"]
pub mod change_building_t1_given_s;
#[path = "../✏️change-building-ct/🦀️.rs"]
pub mod change_building_ct;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
//#endregion 🪢️Mounts
