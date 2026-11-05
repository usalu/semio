//! 🏗️ EN 1993 crane runway action element mutation kinds.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-crane-action/🦀️.rs"]
pub mod insert_crane_action;
#[path = "../➖️remove-crane-action/🦀️.rs"]
pub mod remove_crane_action;
#[path = "../🏗️change-crane-action-load-case-id/🦀️.rs"]
pub mod change_crane_action_load_case_id;
#[path = "../🏗️change-crane-action-force/🦀️.rs"]
pub mod change_crane_action_force;
//#endregion 🪢️Mounts
