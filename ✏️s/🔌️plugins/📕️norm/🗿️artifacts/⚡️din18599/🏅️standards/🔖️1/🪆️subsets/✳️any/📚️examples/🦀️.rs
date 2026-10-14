//! 📚️ DIN V 18599 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "✅️compliant-detached/🦀️.rs"]
pub mod compliant_detached;
#[path = "🏢️compliant-two-zone/🦀️.rs"]
pub mod compliant_two_zone;
#[path = "❄️cooled-office/🦀️.rs"]
pub mod cooled_office;
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[path = "❌️noncompliant-detached/🦀️.rs"]
pub mod noncompliant_detached;
//#endregion 🪢️Mounts
