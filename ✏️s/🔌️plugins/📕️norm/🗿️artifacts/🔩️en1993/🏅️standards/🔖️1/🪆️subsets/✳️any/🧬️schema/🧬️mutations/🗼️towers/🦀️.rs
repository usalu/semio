//! 🗼️ EN 1993 towers mutation leaves.

//#region 🪢️Mounts
#[path = "../➕️insert-pile/🦀️.rs"]
pub mod insert_pile;
#[path = "../➕️insert-tower-leg/🦀️.rs"]
pub mod insert_tower_leg;
#[path = "../➖️remove-pile/🦀️.rs"]
pub mod remove_pile;
#[path = "../➖️remove-tower-leg/🦀️.rs"]
pub mod remove_tower_leg;
#[path = "../🪵️update-pile-inputs/🦀️.rs"]
pub mod update_pile_inputs;
#[path = "../🗼️update-tower-inputs/🦀️.rs"]
pub mod update_tower_inputs;
//#endregion 🪢️Mounts

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
