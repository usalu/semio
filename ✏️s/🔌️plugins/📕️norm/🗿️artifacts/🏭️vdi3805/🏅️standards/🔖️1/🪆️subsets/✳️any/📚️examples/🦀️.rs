//! 📚️ VDI 3805 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "📄️blatt-16/🦀️.rs"]
pub mod blatt_16;
#[path = "📄️blatt-19/🦀️.rs"]
pub mod blatt_19;
#[path = "📄️blatt-22/🦀️.rs"]
pub mod blatt_22;
#[path = "📄️blatt-53/🦀️.rs"]
pub mod blatt_53;
#[path = "📄️blatt-60/🦀️.rs"]
pub mod blatt_60;
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[path = "🌶️nonconforming/🦀️.rs"]
pub mod nonconforming;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_vdi3805_examples_blatt_3_to_8::{blatt_3, blatt_4, blatt_5, blatt_6, blatt_7, blatt_8};
