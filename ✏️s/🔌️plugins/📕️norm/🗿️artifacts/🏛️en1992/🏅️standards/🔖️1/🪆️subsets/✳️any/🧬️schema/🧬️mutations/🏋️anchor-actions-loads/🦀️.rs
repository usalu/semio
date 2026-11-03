//! 🏋️ EN 1992 anchor actions loads mutation leaves: anchor action g k line, anchor action point force, anchor action q k line.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-anchor-action-gk-line/🦀️.rs"]
pub mod change_anchor_action_g_k_line;
#[path = "../🏋️change-anchor-action-qk-line/🦀️.rs"]
pub mod change_anchor_action_q_k_line;
#[path = "../🏋️change-anchor-action-point-force/🦀️.rs"]
pub mod change_anchor_action_point_force;
//#endregion 🪢️Mounts
