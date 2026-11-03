//! ✖️ EN 1995 bracing-layout mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✖️change-bracing-kind/🦀️.rs"]
pub mod change_bracing_kind;
#[path = "../✖️change-bracing-member/🦀️.rs"]
pub mod change_bracing_member;
#[path = "../✖️change-bracing-stabilised-count/🦀️.rs"]
pub mod change_bracing_stabilised_count;
#[path = "../✖️change-bracing-spacing/🦀️.rs"]
pub mod change_bracing_spacing;
#[path = "../✖️change-bracing-length/🦀️.rs"]
pub mod change_bracing_length;
//#endregion 🪢️Mounts
