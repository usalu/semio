//! 🏠️ EN 1995 floor-stiffness mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏠️change-floor-member/🦀️.rs"]
pub mod change_floor_member;
#[path = "../🏠️change-floor-span/🦀️.rs"]
pub mod change_floor_span;
#[path = "../🏠️change-floor-width/🦀️.rs"]
pub mod change_floor_width;
#[path = "../🏠️change-floor-joist-spacing/🦀️.rs"]
pub mod change_floor_joist_spacing;
#[path = "../🏠️change-floor-ei-l/🦀️.rs"]
pub mod change_floor_ei_l;
#[path = "../🏠️change-floor-ei-b/🦀️.rs"]
pub mod change_floor_ei_b;
//#endregion 🪢️Mounts
