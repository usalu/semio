//! 🪜️ VDI 3805 nonconforming example documents: every failing Blatt (part 2).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❌️blatt-19-fail/🦀️.rs"]
pub mod blatt_19_fail;
#[path = "../❌️blatt-4-fail/🦀️.rs"]
pub mod blatt_4_fail;
#[path = "../❌️blatt-5-fail/🦀️.rs"]
pub mod blatt_5_fail;
#[path = "../❌️blatt-6-fail/🦀️.rs"]
pub mod blatt_6_fail;
#[path = "../❌️blatt-8-fail/🦀️.rs"]
pub mod blatt_8_fail;
//#endregion 🪢️Mounts
