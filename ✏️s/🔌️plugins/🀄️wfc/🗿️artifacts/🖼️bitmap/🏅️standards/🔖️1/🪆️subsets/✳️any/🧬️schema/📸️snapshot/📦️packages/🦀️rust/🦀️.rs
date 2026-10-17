//! 📸️ snapshot of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::viewer::bitmap::modes::view::artifact_items_3::{BITMAP_LAYERS_JSON_BUDGET_BYTES, bitmap_layers_at_block, bitmap_layers_json, bitmap_rect_layer};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
