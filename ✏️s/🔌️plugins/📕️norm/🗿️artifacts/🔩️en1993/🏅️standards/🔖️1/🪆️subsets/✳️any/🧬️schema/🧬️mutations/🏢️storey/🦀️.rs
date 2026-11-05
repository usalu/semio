//! 🏢️ EN 1993 storey mutation leaves: member id, label, height and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏢️change-storey-member-id/🦀️.rs"]
pub mod change_storey_member_id;
#[path = "../🏢️change-storey-label/🦀️.rs"]
pub mod change_storey_label;
#[path = "../🏢️change-storey-height/🦀️.rs"]
pub mod change_storey_height;
#[path = "../🏢️change-storey-vertical-load/🦀️.rs"]
pub mod change_storey_vertical_load;
#[path = "../🏢️change-storey-horizontal-load/🦀️.rs"]
pub mod change_storey_horizontal_load;
#[path = "../🏢️change-storey-drift/🦀️.rs"]
pub mod change_storey_drift;
//#endregion 🪢️Mounts
