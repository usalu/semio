//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-length/🦀️.rs"]
pub mod change_member_length;
#[path = "../🏗️change-member-support/🦀️.rs"]
pub mod change_member_support;
#[path = "../🏗️change-member-ltb-length/🦀️.rs"]
pub mod change_member_ltb_length;
#[path = "../🏗️change-member-c1/🦀️.rs"]
pub mod change_member_c1;
#[path = "../🏗️change-member-restrained-ltb/🦀️.rs"]
pub mod change_member_restrained_ltb;
//#endregion 🪢️Mounts
