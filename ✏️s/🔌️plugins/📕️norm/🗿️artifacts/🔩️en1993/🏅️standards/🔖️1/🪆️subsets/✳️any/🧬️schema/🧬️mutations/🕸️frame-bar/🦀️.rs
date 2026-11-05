//! 🕸️ EN 1993 frame bar mutation leaves: member id, label, start node id and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🕸️change-frame-bar-member-id/🦀️.rs"]
pub mod change_frame_bar_member_id;
#[path = "../🕸️change-frame-bar-label/🦀️.rs"]
pub mod change_frame_bar_label;
#[path = "../🕸️change-frame-bar-start-node-id/🦀️.rs"]
pub mod change_frame_bar_start_node_id;
#[path = "../🕸️change-frame-bar-end-node-id/🦀️.rs"]
pub mod change_frame_bar_end_node_id;
#[path = "../🕸️change-frame-bar-release-start/🦀️.rs"]
pub mod change_frame_bar_release_start;
#[path = "../🕸️change-frame-bar-release-end/🦀️.rs"]
pub mod change_frame_bar_release_end;
//#endregion 🪢️Mounts
