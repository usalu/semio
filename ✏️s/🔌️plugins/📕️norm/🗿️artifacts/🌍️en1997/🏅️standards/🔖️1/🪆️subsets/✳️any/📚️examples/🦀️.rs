//! 📚️ EN 1997 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏗️compliant/🦀️.rs"]
pub mod compliant;
#[cfg(test)]
#[path = "🏗️compliant/🧪️tests/🧩️example/🦀️.rs"]
mod example_compliant;
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
#[cfg(test)]
#[path = "🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod example_demo;
#[path = "🚨️noncompliant/🦀️.rs"]
pub mod noncompliant;
#[cfg(test)]
#[path = "🚨️noncompliant/🧪️tests/🧩️example/🦀️.rs"]
mod example_noncompliant;
//#endregion 🪢️Mounts
