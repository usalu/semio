//! 🏗️ EN 1990 members horizontal mutation leaves: member frame drift, member frame height, member storey drift and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-frame-height/🦀️.rs"]
pub mod change_member_frame_height;
#[path = "../🏗️change-member-frame-drift/🦀️.rs"]
pub mod change_member_frame_drift;
#[path = "../🏗️change-member-storey-height/🦀️.rs"]
pub mod change_member_storey_height;
#[path = "../🏗️change-member-storey-drift/🦀️.rs"]
pub mod change_member_storey_drift;
//#endregion 🪢️Mounts
