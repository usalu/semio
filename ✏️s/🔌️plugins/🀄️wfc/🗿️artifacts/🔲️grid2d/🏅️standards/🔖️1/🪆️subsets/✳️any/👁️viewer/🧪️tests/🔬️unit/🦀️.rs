//! 🧪️ Viewer laws — a read-only surface that boots the same scene the editor does.

use super::*;

#[test]
fn the_viewer_builds_a_definition_for_the_viewer_role() {
    let definition = create_grid2d_viewer();
    assert_eq!(definition.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(definition.dialect, WFC_GRID2D_DIALECT.into());
}

#[test]
fn the_viewer_declares_one_read_only_window() {
    let definition = create_grid2d_viewer();
    assert_eq!(definition.window_kinds.len(), 1);
    let window = definition.window_kinds.first();
    assert_eq!(window.id, preview::WINDOW_KIND_ID);
    assert!(
        window.actions.iter().all(|action| action.kind != semio_framework_plugin::ActionKind::Mutation),
        "a viewer may carry framework-reserved rows but never a mutation verb"
    );
    assert!(window.utilities.is_empty());
}

#[test]
fn the_viewer_boots_the_same_committed_example_the_editor_does() {
    assert_eq!(<Grid2dViewer as ArtifactViewer>::initial_snapshot(), crate::examples::grid2d::pipes::document());
}

/// 👁️ Every row of the view command channel is a HOST-dispatched verb the window must declare —
/// the ids the shell's navbar picker and `Canvas2dHost` send — and each round trips its own wire.
#[test]
fn the_view_command_channel_carries_exactly_the_host_dispatched_verbs() {
    assert_eq!(GRID2D_VIEW_TOOL_IDS, <Grid2dViewCommand as protocol::OpBinary>::TOOL_JOB_IDS, "the proof catalog and the wire ids are one declaration");
    let definition = create_grid2d_viewer();
    let window = definition.window_kinds.first();
    for action in GRID2D_VIEW_TOOL_IDS {
        assert!(window.actions.iter().any(|declared| &declared.id == action), "{action} is dispatched at this pane but declared by no window");
        let command = <Grid2dViewer as ArtifactViewer>::command_from_action(action, None).unwrap_or_else(|error| panic!("{action}: {error:?}"));
        assert_eq!(grid2d_view_command_id(&command), *action);
        let bytes = <Grid2dViewCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
        assert_eq!(<Grid2dViewCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode"), command);
    }
    assert!(<Grid2dViewer as ArtifactViewer>::command_from_action("pin-cell", None).is_err(), "a mutation verb never reaches a read-only surface");
}

#[test]
fn every_bundled_example_renders_a_non_empty_read_only_canvas() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses");
        preview::render(&document).unwrap_or_else(|error| panic!("{}: the viewer must render: {error:?}", source.id()));
        assert!(preview::scene(&document).layers_json.len() > 64, "{}: the read-only canvas carries no layers", source.id());
    }
}

#[test]
fn a_pinned_cell_draws_its_tile_media_while_a_masked_cell_draws_nothing() {
    let document = crate::examples::grid2d::pipes::document();
    let layers: Vec<serde_json::Value> = serde_json::from_str(&preview::layers_json(&document)).expect("layers are real json");
    assert!(layers.iter().any(|layer| layer["id"].as_str().is_some_and(|id| id.starts_with("pin-0-0"))), "the pinned cell draws its tile");
    assert!(!layers.iter().any(|layer| layer["id"] == "cell-5-5"), "the masked cell draws nothing");
}
