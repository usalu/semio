//! 📚️ DIN EN 16798 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️compliant-office/🦀️.rs"]
pub mod compliant_office;
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[path = "⚠️noncompliant-office/🦀️.rs"]
pub mod noncompliant_office;
#[path = "🏠️residential-method3/🦀️.rs"]
pub mod residential_method3;
//#endregion 🪢️Mounts
