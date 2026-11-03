//! 🪵️ EN 1993 pile action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-pile-action/🦀️.rs"]
pub mod insert_pile_action;
#[path = "../➖️remove-pile-action/🦀️.rs"]
pub mod remove_pile_action;
#[path = "../🪵️change-pile-action-load-case-id/🦀️.rs"]
pub mod change_pile_action_load_case_id;
#[path = "../🪵️change-pile-action-force/🦀️.rs"]
pub mod change_pile_action_force;
//#endregion 🪢️Mounts
