//! 📍️ EN 1995 fastener-positions mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➕️insert-fastener-position/🦀️.rs"]
pub mod insert_fastener_position;
#[path = "../➖️remove-fastener-position/🦀️.rs"]
pub mod remove_fastener_position;
#[path = "../📍️change-fastener-position-x/🦀️.rs"]
pub mod change_fastener_position_x;
#[path = "../📍️change-fastener-position-y/🦀️.rs"]
pub mod change_fastener_position_y;
//#endregion 🪢️Mounts
