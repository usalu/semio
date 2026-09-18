//! 🧪️ Viewer surface — read-only by construction, bound to the same dialect, two windows.

use super::*;

#[test]
fn the_manifest_binds_both_windows() {
    let definition = create_bitmap_viewer();
    assert_eq!(definition.id, "s.wfc.bitmap@1/*#viewer");
    assert_eq!(definition.window_kinds.len(), 2);
}

#[test]
fn the_viewer_shares_the_editors_dialect_and_boot_example() {
    assert_eq!(<BitmapViewer as ArtifactViewer>::DIALECT, WFC_BITMAP_DIALECT);
    assert_eq!(<BitmapViewer as ArtifactViewer>::initial_snapshot(), crate::examples::rooms_16::snapshot());
}

#[test]
fn the_only_command_is_inert() {
    let bytes = protocol::OpBinary::encode_op(&BitmapViewCommand::Noop).expect("noop encodes");
    assert!(bytes.is_empty());
    assert_eq!(<BitmapViewCommand as protocol::OpBinary>::decode_op(&bytes).expect("noop decodes"), BitmapViewCommand::Noop);
}
