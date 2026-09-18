//! 🧪️ Viewer surface laws: the app id is canonical, the surface is structurally read-only, and it
//! boots on the same example the editor does.

use super::*;
use semio_framework_plugin::ArtifactViewer;

#[test]
fn the_viewer_app_id_is_the_canonical_surface_id() {
    let definition = create_wfc3d_viewer();
    assert_eq!(definition.id, "s.wfc.wfc3d@1/*#viewer");
    assert_eq!(definition.breadcrumb, vec!["semio".to_string(), "wfc".to_string(), "3d".to_string()]);
}

#[test]
fn the_viewer_declares_exactly_one_read_only_window() {
    let definition = create_wfc3d_viewer();
    assert_eq!(definition.window_kinds.len(), 1);
    assert_eq!(definition.window_kinds[0].id, preview::WFC_3D_VIEW_WINDOW);
    assert!(
        definition.window_kinds[0].actions.iter().all(|action| action.kind != semio_framework_plugin::ActionKind::Mutation),
        "a viewer window declares no MUTATING action; the shell's own chrome verbs are injected by the builder and are not this surface's"
    );
}

#[test]
fn the_viewer_and_the_editor_share_one_dialect() {
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::DIALECT, WFC3D_DIALECT);
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::DOCUMENT_SCHEMA, WFC3D_DOCUMENT_SCHEMA);
}

#[test]
fn the_viewer_boots_on_a_real_document() {
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::initial_snapshot(), crate::examples::two_room_corridor::snapshot());
}

#[test]
fn an_unknown_body_key_renders_a_label_instead_of_failing() {
    assert!(render_body("nope", &crate::examples::two_room_corridor::snapshot()).is_ok());
}

/// 👁️ The read-only body renders for every bundled example — the same statement the editor makes for
/// its own two windows.
#[test]
fn the_view_body_renders_for_every_example() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let tree = render_body(preview::WFC_3D_VIEW_BODY, &document).expect("the viewer body renders");
        assert!(!format!("{tree:?}").is_empty());
    }
}
