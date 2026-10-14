//! 📚️ EN 1999 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏠️aluminium-roof-purlin/🦀️.rs"]
pub mod aluminium_roof_purlin;
#[cfg(test)]
#[path = "🏠️aluminium-roof-purlin/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "🏚️noncompliant-multi-fail/🦀️.rs"]
pub mod noncompliant_multi_fail;
//#endregion 🪢️Mounts
