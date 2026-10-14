//! 📚️ EN 1991 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️de-office-compliant/🦀️.rs"]
pub mod de_office_compliant;
#[cfg(test)]
#[path = "🏢️de-office-compliant/🧪️tests/📚️example/🦀️.rs"]
mod example_de_office_compliant;
#[cfg(test)]
#[path = "⚠️multi-fail-noncompliant/🧪️tests/📚️example/🦀️.rs"]
mod example_multi_fail_noncompliant;
#[path = "🧬️subjects/🦀️.rs"]
pub mod example_subjects;
#[path = "⚠️multi-fail-noncompliant/🦀️.rs"]
pub mod multi_fail_noncompliant;
//#endregion 🪢️Mounts
