//! 🏗️ EN 1990 members identity mutation leaves: member label de, member label en, member precamber and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-label-en/🦀️.rs"]
pub mod change_member_label_en;
#[path = "../🏗️change-member-label-de/🦀️.rs"]
pub mod change_member_label_de;
#[path = "../🏗️change-member-span/🦀️.rs"]
pub mod change_member_span;
#[path = "../🏗️change-member-precamber/🦀️.rs"]
pub mod change_member_precamber;
//#endregion 🪢️Mounts
