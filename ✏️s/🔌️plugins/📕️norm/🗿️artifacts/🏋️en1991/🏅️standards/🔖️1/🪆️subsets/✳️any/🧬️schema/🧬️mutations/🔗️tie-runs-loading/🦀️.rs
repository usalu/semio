//! 🔗 EN 1991 tie runs loading mutation leaves: tie run dead load, tie run imposed load, tie run psi and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-tie-run-spacing/🦀️.rs"]
pub mod change_tie_run_spacing;
#[path = "../🔗️change-tie-run-span/🦀️.rs"]
pub mod change_tie_run_span;
#[path = "../🔗️change-tie-run-dead-load/🦀️.rs"]
pub mod change_tie_run_dead_load;
#[path = "../🔗️change-tie-run-imposed-load/🦀️.rs"]
pub mod change_tie_run_imposed_load;
#[path = "../🔗️change-tie-run-psi/🦀️.rs"]
pub mod change_tie_run_psi;
//#endregion 🪢️Mounts
