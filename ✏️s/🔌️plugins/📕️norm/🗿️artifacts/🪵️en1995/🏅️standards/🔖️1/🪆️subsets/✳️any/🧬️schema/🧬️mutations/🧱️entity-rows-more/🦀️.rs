//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-fastener/🦀️.rs"]
pub mod insert_fastener;
#[path = "../➖️remove-fastener/🦀️.rs"]
pub mod remove_fastener;
#[path = "../➕️insert-panel/🦀️.rs"]
pub mod insert_panel;
#[path = "../➖️remove-panel/🦀️.rs"]
pub mod remove_panel;
#[path = "../➕️insert-floor/🦀️.rs"]
pub mod insert_floor;
#[path = "../➖️remove-floor/🦀️.rs"]
pub mod remove_floor;
//#endregion 🪢️Mounts
