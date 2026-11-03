//! 🏢️ EN 1993 row mutation leaves: storey, restraint, built-up-member.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-storey/🦀️.rs"]
pub mod insert_storey;
#[path = "../➖️remove-storey/🦀️.rs"]
pub mod remove_storey;
#[path = "../➕️insert-restraint/🦀️.rs"]
pub mod insert_restraint;
#[path = "../➖️remove-restraint/🦀️.rs"]
pub mod remove_restraint;
#[path = "../➕️insert-built-up-member/🦀️.rs"]
pub mod insert_built_up_member;
#[path = "../➖️remove-built-up-member/🦀️.rs"]
pub mod remove_built_up_member;
//#endregion 🪢️Mounts
