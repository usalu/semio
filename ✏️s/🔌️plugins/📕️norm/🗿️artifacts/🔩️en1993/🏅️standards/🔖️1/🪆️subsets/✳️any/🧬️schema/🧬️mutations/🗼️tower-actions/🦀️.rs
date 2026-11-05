//! 🗼️ EN 1993 tower leg action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-tower-action/🦀️.rs"]
pub mod insert_tower_action;
#[path = "../➖️remove-tower-action/🦀️.rs"]
pub mod remove_tower_action;
#[path = "../🗼️change-tower-action-load-case-id/🦀️.rs"]
pub mod change_tower_action_load_case_id;
#[path = "../🗼️change-tower-action-force/🦀️.rs"]
pub mod change_tower_action_force;
//#endregion 🪢️Mounts
