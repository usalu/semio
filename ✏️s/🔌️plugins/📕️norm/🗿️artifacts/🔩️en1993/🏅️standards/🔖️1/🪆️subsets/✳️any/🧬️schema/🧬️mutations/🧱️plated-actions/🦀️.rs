//! 🧱️ EN 1993 plated panel action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-plated-action/🦀️.rs"]
pub mod insert_plated_action;
#[path = "../➖️remove-plated-action/🦀️.rs"]
pub mod remove_plated_action;
#[path = "../🧱️change-plated-action-load-case-id/🦀️.rs"]
pub mod change_plated_action_load_case_id;
#[path = "../🧱️change-plated-action-force/🦀️.rs"]
pub mod change_plated_action_force;
//#endregion 🪢️Mounts
