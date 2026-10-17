//! ❌️ VDI 3805 nonconforming example documents: every failing Blatt.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../❌️blatt-16-fail/🦀️.rs"]
pub mod blatt_16_fail;
#[path = "../❌️blatt-3-fail/🦀️.rs"]
pub mod blatt_3_fail;
#[path = "../❌️blatt-53-fail/🦀️.rs"]
pub mod blatt_53_fail;
#[path = "../❌️blatt-60-fail/🦀️.rs"]
pub mod blatt_60_fail;
#[path = "../❌️blatt-7-fail/🦀️.rs"]
pub mod blatt_7_fail;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_vdi3805_examples_failing_2::{blatt_19_fail, blatt_4_fail, blatt_5_fail, blatt_6_fail, blatt_8_fail};
