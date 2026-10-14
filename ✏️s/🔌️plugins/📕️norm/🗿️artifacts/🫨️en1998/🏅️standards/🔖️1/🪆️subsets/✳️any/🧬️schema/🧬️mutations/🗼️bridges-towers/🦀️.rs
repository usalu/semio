//! 🗼️ EN 1998 bridges towers mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛑️change-bridge-v-rd-n/🦀️.rs"]
pub mod change_bridge_v_rd_n;
#[path = "../↪️change-tower-m-rd-nm/🦀️.rs"]
pub mod change_tower_m_rd_nm;
#[path = "../🌉insert-bridge/🦀️.rs"]
pub mod insert_bridge;
#[path = "../🗼insert-tower/🦀️.rs"]
pub mod insert_tower;
#[path = "../➖️remove-bridge/🦀️.rs"]
pub mod remove_bridge;
#[path = "../➖️remove-tower/🦀️.rs"]
pub mod remove_tower;
//#endregion 🪢️Mounts
