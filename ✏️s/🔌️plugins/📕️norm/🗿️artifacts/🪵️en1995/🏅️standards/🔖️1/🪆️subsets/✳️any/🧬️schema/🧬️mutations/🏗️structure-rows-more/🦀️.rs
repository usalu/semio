//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-bracing/🦀️.rs"]
pub mod insert_bracing;
#[path = "../➖️remove-bracing/🦀️.rs"]
pub mod remove_bracing;
#[path = "../➕️insert-deck/🦀️.rs"]
pub mod insert_deck;
#[path = "../➖️remove-deck/🦀️.rs"]
pub mod remove_deck;
//#endregion 🪢️Mounts
