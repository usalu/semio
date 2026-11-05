//! 🌾 EN 1991 bulk solids mutation leaves: bulk solid, bulk solid internal friction.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🌾️change-bulk-solid-internal-friction/🦀️.rs"]
pub mod change_bulk_solid_internal_friction;
#[path = "../🌾️insert-bulk-solid/🦀️.rs"]
pub mod insert_bulk_solid;
#[path = "../🌾️remove-bulk-solid/🦀️.rs"]
pub mod remove_bulk_solid;
//#endregion 🪢️Mounts
