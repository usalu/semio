//! 🔖️ v1 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::artifact_items::{BimApplication, artifact, definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
