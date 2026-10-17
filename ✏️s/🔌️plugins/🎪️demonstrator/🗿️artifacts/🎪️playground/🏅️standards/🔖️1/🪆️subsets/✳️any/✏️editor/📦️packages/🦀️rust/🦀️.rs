//! ✏️ editor of `semio-s-artifact-demonstrator-playground` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use semio_s_artifact_demonstrator_playground_schema::{PLAYGROUND_DOCUMENT_SCHEMA};
pub use crate::editor::playground::artifact_items_2::{PLAYGROUND_DIALECT};
pub use crate::editor::playground::artifact_items_3::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
