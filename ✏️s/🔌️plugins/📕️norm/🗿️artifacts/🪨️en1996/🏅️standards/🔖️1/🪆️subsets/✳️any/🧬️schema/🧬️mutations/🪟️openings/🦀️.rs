//! 🪟️ EN 1996 openings mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪟change-opening-height/🦀️.rs"]
pub mod change_opening_height;
#[path = "../🪟change-opening-sill/🦀️.rs"]
pub mod change_opening_sill;
#[path = "../🪟change-opening-width/🦀️.rs"]
pub mod change_opening_width;
#[path = "../➕️insert-opening/🦀️.rs"]
pub mod insert_opening;
#[path = "../➖️remove-opening/🦀️.rs"]
pub mod remove_opening;
//#endregion 🪢️Mounts
