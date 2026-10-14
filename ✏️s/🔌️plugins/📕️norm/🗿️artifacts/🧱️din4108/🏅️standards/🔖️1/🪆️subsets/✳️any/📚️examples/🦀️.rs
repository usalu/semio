//! 📚️ DIN 4108 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[path = "🎬️failing-thin-insulation/🦀️.rs"]
pub mod failing_thin_insulation;
//#endregion 🪢️Mounts
