//! 📚️ EN 1992 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️compliant-office-frame/🦀️.rs"]
pub mod compliant_office_frame;
#[cfg(test)]
#[path = "🏢️compliant-office-frame/🧪️tests/🧩️example/🦀️.rs"]
mod compliant_office_frame_example_tests;
#[path = "🧵️compliant-prestressed-beam/🦀️.rs"]
mod compliant_prestressed_beam;
#[cfg(test)]
#[path = "🧵️compliant-prestressed-beam/🧪️tests/🧩️example/🦀️.rs"]
mod compliant_prestressed_beam_example_tests;
#[cfg(test)]
#[path = "🛢️liquid-retaining-fem-anchor/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "💥️failing-prestressed-beam/🦀️.rs"]
mod failing_prestressed_beam;
#[cfg(test)]
#[path = "💥️failing-prestressed-beam/🧪️tests/🧩️example/🦀️.rs"]
mod failing_prestressed_beam_example_tests;
#[path = "❌️failing-under-reinforced/🦀️.rs"]
pub mod failing_under_reinforced;
#[cfg(test)]
#[path = "❌️failing-under-reinforced/🧪️tests/🧩️example/🦀️.rs"]
mod failing_under_reinforced_example_tests;
#[path = "🛢️liquid-retaining-fem-anchor/🦀️.rs"]
pub mod liquid_retaining_fem_anchor;
//#endregion 🪢️Mounts
