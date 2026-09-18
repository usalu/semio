//! 🧪️ The viewer surface — identity and read-only laws.

use crate::viewer::wfc2d::{create_wfc2d_viewer, Wfc2dViewer};
use semio_framework_plugin::ArtifactViewer;

#[test]
fn viewer_binds_the_artifact_dialect() {
    assert_eq!(Wfc2dViewer::DIALECT.artifact_kind, crate::WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(Wfc2dViewer::DOCUMENT_SCHEMA, crate::WFC_2D_DOCUMENT_SCHEMA);
}

/// 👁️ Editor and viewer boot the SAME committed example, so one board opens as one scene in both.
#[test]
fn viewer_boots_the_same_example_as_the_editor() {
    assert_eq!(Wfc2dViewer::initial_snapshot(), crate::examples::two_room_corridor::document());
}

#[test]
fn viewer_manifest_declares_one_read_only_window() {
    let definition = create_wfc2d_viewer();
    assert_eq!(definition.id, "s.wfc.wfc2d@1/*#viewer");
    assert_eq!(definition.window_kinds.len(), 1);
}
