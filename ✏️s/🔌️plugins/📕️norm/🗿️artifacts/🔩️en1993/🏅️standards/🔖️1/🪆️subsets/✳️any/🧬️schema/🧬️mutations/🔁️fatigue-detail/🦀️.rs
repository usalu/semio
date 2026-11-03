//! 🔁️ EN 1993 fatigue detail mutation leaves: member id, category, method.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔁️change-fatigue-member-id/🦀️.rs"]
pub mod change_fatigue_member_id;
#[path = "../🔁️change-fatigue-category/🦀️.rs"]
pub mod change_fatigue_category;
#[path = "../🔁️change-fatigue-method/🦀️.rs"]
pub mod change_fatigue_method;
#[path = "../🔁️change-fatigue-consequence/🦀️.rs"]
pub mod change_fatigue_consequence;
//#endregion 🪢️Mounts
