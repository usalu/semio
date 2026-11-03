//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌉️change-member-span/🦀️.rs"]
pub mod change_member_span;
#[path = "../↔️change-member-width/🦀️.rs"]
pub mod change_member_width;
#[path = "../➕️insert-member/🦀️.rs"]
pub mod insert_member;
#[path = "../➖️remove-member/🦀️.rs"]
pub mod remove_member;
#[path = "../🔀️change-member-order/🦀️.rs"]
pub mod change_member_order;
//#endregion 🪢️Mounts
