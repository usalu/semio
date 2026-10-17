//! ⚙️ operations of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::WriterDiff;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::WriterMutation;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_writer_writer_schema::{WRITER_DOCUMENT_SCHEMA, writer_snapshot_with_text, writer_text};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
