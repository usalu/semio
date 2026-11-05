//! 🧰️ EN 1996 confined masonry elements mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧰️change-confined-position/🦀️.rs"]
pub mod change_confined_position;
#[path = "../🧰️change-confined-width/🦀️.rs"]
pub mod change_confined_width;
#[path = "../🧰️change-confined-depth/🦀️.rs"]
pub mod change_confined_depth;
#[path = "../🧰️change-confined-steel/🦀️.rs"]
pub mod change_confined_steel;
#[path = "../➕️insert-confined/🦀️.rs"]
pub mod insert_confined;
#[path = "../➖️remove-confined/🦀️.rs"]
pub mod remove_confined;
//#endregion 🪢️Mounts
