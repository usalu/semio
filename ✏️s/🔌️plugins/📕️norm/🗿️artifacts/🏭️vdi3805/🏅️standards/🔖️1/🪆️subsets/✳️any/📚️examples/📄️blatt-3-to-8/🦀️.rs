//! 📄️ VDI 3805 conforming example documents: Blatt 3, 4, 5, 6, 7 and 8.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📄️blatt-3/🦀️.rs"]
pub mod blatt_3;
#[path = "../📄️blatt-4/🦀️.rs"]
pub mod blatt_4;
#[path = "../📄️blatt-5/🦀️.rs"]
pub mod blatt_5;
#[path = "../📄️blatt-6/🦀️.rs"]
pub mod blatt_6;
#[path = "../📄️blatt-7/🦀️.rs"]
pub mod blatt_7;
#[path = "../📄️blatt-8/🦀️.rs"]
pub mod blatt_8;
//#endregion 🪢️Mounts
