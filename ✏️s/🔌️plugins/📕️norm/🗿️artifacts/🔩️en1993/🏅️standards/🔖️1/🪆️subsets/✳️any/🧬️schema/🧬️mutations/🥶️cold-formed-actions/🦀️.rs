//! 🥶️ EN 1993 cold-formed action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-cf-action/🦀️.rs"]
pub mod insert_cf_action;
#[path = "../➖️remove-cf-action/🦀️.rs"]
pub mod remove_cf_action;
#[path = "../🥶️change-cf-action-load-case-id/🦀️.rs"]
pub mod change_cf_action_load_case_id;
#[path = "../🥶️change-cf-action-force/🦀️.rs"]
pub mod change_cf_action_force;
//#endregion 🪢️Mounts
