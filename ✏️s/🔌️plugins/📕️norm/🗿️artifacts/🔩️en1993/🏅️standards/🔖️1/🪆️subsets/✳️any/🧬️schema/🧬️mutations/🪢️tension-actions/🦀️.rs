//! 🪢️ EN 1993 tension component action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-tension-action/🦀️.rs"]
pub mod insert_tension_action;
#[path = "../➖️remove-tension-action/🦀️.rs"]
pub mod remove_tension_action;
#[path = "../🪢️change-tension-action-load-case-id/🦀️.rs"]
pub mod change_tension_action_load_case_id;
#[path = "../🪢️change-tension-action-force/🦀️.rs"]
pub mod change_tension_action_force;
//#endregion 🪢️Mounts
