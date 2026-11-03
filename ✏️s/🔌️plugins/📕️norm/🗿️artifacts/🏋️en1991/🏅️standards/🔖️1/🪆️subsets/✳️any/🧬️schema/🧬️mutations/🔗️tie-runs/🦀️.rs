//! 🔗 EN 1991 tie runs mutation leaves: tie run.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️insert-tie-run/🦀️.rs"]
pub mod insert_tie_run;
#[path = "../🔗️remove-tie-run/🦀️.rs"]
pub mod remove_tie_run;
//#endregion 🪢️Mounts
