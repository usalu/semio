//! 🔬️ tests-2 of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
pub use crate::editor::writer::modes::edit::windows::main::config::WriterCamera;
pub use crate::editor::writer::modes::edit::windows::main::transient::WriterEditorSelection;
pub use crate::editor::writer::modes::edit::windows::main::config::default_camera;
pub use crate::editor::writer::modes::edit::windows::main::config::default_zoom;
pub use crate::editor::writer::modes::edit::windows::main::config::WriterEditorSettings;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::WriterMutation;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_writer_writer_schema::{WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA, genesis_writer_child_pack, writer_snapshot_with_text, writer_text};
pub use crate::editor::writer::artifact_items_5::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
