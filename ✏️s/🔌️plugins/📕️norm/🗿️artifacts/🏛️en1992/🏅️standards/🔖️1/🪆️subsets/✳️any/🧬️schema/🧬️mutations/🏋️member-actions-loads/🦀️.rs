//! 🏋️ EN 1992 member actions loads mutation leaves: action g k line, action point force, action q k line and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-action-gk-line/🦀️.rs"]
pub mod change_action_g_k_line;
#[path = "../🏋️change-action-qk-line/🦀️.rs"]
pub mod change_action_q_k_line;
#[path = "../🏋️change-action-point-force/🦀️.rs"]
pub mod change_action_point_force;
#[path = "../🏋️change-action-tk/🦀️.rs"]
pub mod change_action_t_k;
#[path = "../🏋️change-action-vk-punch/🦀️.rs"]
pub mod change_action_v_k_punch;
//#endregion 🪢️Mounts
