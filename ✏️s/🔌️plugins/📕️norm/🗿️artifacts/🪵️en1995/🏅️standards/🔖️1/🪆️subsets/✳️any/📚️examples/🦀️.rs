//! 📚️ EN 1995 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🌉️glulam-footbridge/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[cfg(test)]
#[path = "🏠️glulam-floor-beam/🧪️tests/🧩️example/🦀️.rs"]
mod example_glulam_floor_beam;
#[cfg(test)]
#[path = "❌️multi-fail-timber/🧪️tests/🧩️example/🦀️.rs"]
mod example_multi_fail;
#[cfg(test)]
#[path = "⚠️overloaded-footbridge/🧪️tests/🧩️example/🦀️.rs"]
mod example_overloaded_footbridge;
#[path = "🏠️glulam-floor-beam/🦀️.rs"]
pub mod glulam_floor_beam;
#[path = "🌉️glulam-footbridge/🦀️.rs"]
pub mod glulam_footbridge;
#[path = "❌️multi-fail-timber/🦀️.rs"]
pub mod multi_fail_timber;
#[path = "⚠️overloaded-footbridge/🦀️.rs"]
pub mod overloaded_footbridge;
//#endregion 🪢️Mounts

/// 📚️ Every bundled EN 1995 example in catalogue order — compliant first, then non-compliant.
pub fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
    vec![
        glulam_floor_beam::source(),
        glulam_footbridge::source(),
        multi_fail_timber::source(),
        overloaded_footbridge::source(),
    ]
}
