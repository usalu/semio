//! 🏋️ EN 1992 anchor actions resultants mutation leaves: anchor action m k, anchor action n k, anchor action t k and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-anchor-action-mk/🦀️.rs"]
pub mod change_anchor_action_m_k;
#[path = "../🏋️change-anchor-action-nk/🦀️.rs"]
pub mod change_anchor_action_n_k;
#[path = "../🏋️change-anchor-action-vk/🦀️.rs"]
pub mod change_anchor_action_v_k;
#[path = "../🏋️change-anchor-action-tk/🦀️.rs"]
pub mod change_anchor_action_t_k;
#[path = "../🏋️change-anchor-action-vk-punch/🦀️.rs"]
pub mod change_anchor_action_v_k_punch;
//#endregion 🪢️Mounts
