//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➖remove-characteristic-basis/🦀️.rs"]
pub mod remove_characteristic_basis;
#[path = "../➖remove-ground-investigation/🦀️.rs"]
pub mod remove_ground_investigation;
#[path = "../➖remove-pile-group/🦀️.rs"]
pub mod remove_pile_group;
#[path = "../➕insert-characteristic-basis/🦀️.rs"]
pub mod insert_characteristic_basis;
#[path = "../➕insert-ground-investigation/🦀️.rs"]
pub mod insert_ground_investigation;
#[path = "../➕insert-pile-group/🦀️.rs"]
pub mod insert_pile_group;
//#endregion 🪢️Mounts
