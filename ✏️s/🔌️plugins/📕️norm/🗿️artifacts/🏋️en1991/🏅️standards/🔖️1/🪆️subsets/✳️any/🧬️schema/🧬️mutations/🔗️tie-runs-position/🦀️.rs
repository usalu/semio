//! 🔗 EN 1991 tie runs position mutation leaves: tie run kind, tie run provided force, tie run x and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-tie-run-kind/🦀️.rs"]
pub mod change_tie_run_kind;
#[path = "../🔗️change-tie-run-x/🦀️.rs"]
pub mod change_tie_run_x;
#[path = "../🔗️change-tie-run-y/🦀️.rs"]
pub mod change_tie_run_y;
#[path = "../🔗️change-tie-run-provided-force/🦀️.rs"]
pub mod change_tie_run_provided_force;
//#endregion 🪢️Mounts
