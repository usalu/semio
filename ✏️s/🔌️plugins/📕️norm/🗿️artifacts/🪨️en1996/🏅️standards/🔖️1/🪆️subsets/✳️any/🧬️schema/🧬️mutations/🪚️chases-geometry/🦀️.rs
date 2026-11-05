//! 🪚️ EN 1996 chases geometry mutation leaves: chase depth, chase length, chase width and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪚️change-chase-x/🦀️.rs"]
pub mod change_chase_x;
#[path = "../🪚️change-chase-y/🦀️.rs"]
pub mod change_chase_y;
#[path = "../🪚️change-chase-width/🦀️.rs"]
pub mod change_chase_width;
#[path = "../🪚️change-chase-length/🦀️.rs"]
pub mod change_chase_length;
#[path = "../🪚️change-chase-depth/🦀️.rs"]
pub mod change_chase_depth;
//#endregion 🪢️Mounts
