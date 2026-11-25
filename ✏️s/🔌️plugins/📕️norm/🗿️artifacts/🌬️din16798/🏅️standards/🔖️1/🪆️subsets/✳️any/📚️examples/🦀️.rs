//! 📚️ DIN EN 16798 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️compliant-office/🦀️.rs"]
pub mod compliant_office;
#[cfg(test)]
#[path = "🏢️compliant-office/🧪️tests/🧩️example/🦀️.rs"]
mod example_compliant_office;
#[path = "⚠️noncompliant-office/🦀️.rs"]
pub mod noncompliant_office;
#[cfg(test)]
#[path = "⚠️noncompliant-office/🧪️tests/🧩️example/🦀️.rs"]
mod example_noncompliant_office;
#[path = "🏠️residential-method3/🦀️.rs"]
pub mod residential_method3;
#[cfg(test)]
#[path = "🏠️residential-method3/🧪️tests/🧩️example/🦀️.rs"]
mod example_residential_method3;
#[cfg(test)]
#[path = "🧪️tests/🗜️pack-twins/🦀️.rs"]
mod pack_twins;
//#endregion 🪢️Mounts
