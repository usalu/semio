//! 🔬️ Viewer laws — the same dialect as the editor, one read-only window, and a command channel that
//! structurally cannot author a mutation.

use super::*;

#[test]
fn the_manifest_declares_one_read_only_window() {
    let definition = create_grid3d_viewer();
    assert_eq!(definition.id, "s.wfc.grid3d@1/*#viewer");
    assert_eq!(definition.window_kinds.len(), 1);
    let mutating: Vec<&str> = definition.window_kinds[0].actions.iter().filter(|action| matches!(action.kind, semio_framework_plugin::ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
    assert!(mutating.is_empty(), "a viewer window may carry framework chrome actions, but never a mutating one: {mutating:?}");
}

#[test]
fn the_view_command_is_inert_in_both_directions() {
    let command = Grid3dViewCommand::default();
    let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
    assert!(bytes.is_empty());
    assert_eq!(<Grid3dViewCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), Grid3dViewCommand::Noop);
}

#[test]
fn the_viewer_opens_on_the_same_committed_example_the_editor_does() {
    assert_eq!(<Grid3dViewer as ArtifactViewer>::initial_snapshot(), crate::examples::blocks::snapshot());
}
