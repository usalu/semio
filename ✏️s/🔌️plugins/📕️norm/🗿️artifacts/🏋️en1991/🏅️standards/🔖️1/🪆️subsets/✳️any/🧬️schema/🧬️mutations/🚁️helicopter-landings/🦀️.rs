//! 🚁 EN 1991 helicopter landings mutation leaves: helicopter, helicopter assumed force, helicopter height and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🚁️change-helicopter-mass/🦀️.rs"]
pub mod change_helicopter_mass;
#[path = "../🚁️change-helicopter-x/🦀️.rs"]
pub mod change_helicopter_x;
#[path = "../🚁️change-helicopter-y/🦀️.rs"]
pub mod change_helicopter_y;
#[path = "../🚁️change-helicopter-height/🦀️.rs"]
pub mod change_helicopter_height;
#[path = "../🚁️change-helicopter-assumed-force/🦀️.rs"]
pub mod change_helicopter_assumed_force;
#[path = "../🚁️insert-helicopter/🦀️.rs"]
pub mod insert_helicopter;
#[path = "../🚁️remove-helicopter/🦀️.rs"]
pub mod remove_helicopter;
//#endregion 🪢️Mounts
