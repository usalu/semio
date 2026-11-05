//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-bearing-d-rd-m/🦀️.rs"]
pub mod change_bearing_d_rd_m;
#[path = "../✏️change-bearing-seat-length-m/🦀️.rs"]
pub mod change_bearing_seat_length_m;
#[path = "../✏️change-bearing-min-support-m/🦀️.rs"]
pub mod change_bearing_min_support_m;
#[path = "../➕️insert-bearing/🦀️.rs"]
pub mod insert_bearing;
#[path = "../➖️remove-bearing/🦀️.rs"]
pub mod remove_bearing;
//#endregion 🪢️Mounts
