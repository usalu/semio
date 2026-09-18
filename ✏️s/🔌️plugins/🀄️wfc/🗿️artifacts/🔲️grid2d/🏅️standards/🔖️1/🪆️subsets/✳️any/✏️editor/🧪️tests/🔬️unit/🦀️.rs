//! 🧪️ Editor laws — the manifest shape, the action roster, the typed command channel, and the
//! per-pane dispatch every command owes its own window.

use super::*;

#[test]
fn the_editor_builds_a_definition_for_the_editor_role() {
    let definition = create_grid2d_editor();
    assert_eq!(definition.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(definition.dialect, WFC_GRID2D_DIALECT.into());
}

#[test]
fn the_editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Grid2dEditor as ArtifactEditor>::DIALECT, WFC_GRID2D_DIALECT);
    assert_eq!(<Grid2dEditor as ArtifactEditor>::DOCUMENT_SCHEMA, WFC_GRID2D_DOCUMENT_SCHEMA);
}

#[test]
fn the_editor_declares_both_panes_and_a_fifty_fifty_row_layout() {
    let definition = create_grid2d_editor();
    assert!(definition.window_kinds.iter().any(|window| window.id == grid::WINDOW_KIND_ID));
    assert!(definition.window_kinds.iter().any(|window| window.id == preview::WINDOW_KIND_ID));
    let layout = dsl::json::to_json_string(&edit::layout());
    assert!(layout.contains(grid::WINDOW_KIND_ID) && layout.contains(preview::WINDOW_KIND_ID), "{layout}");
    assert!(layout.contains("50"), "the two panes share the row evenly: {layout}");
}

#[test]
fn both_panes_are_localized_en_and_de() {
    let definition = create_grid2d_editor();
    let grid_window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    assert_eq!(grid_window.label, semio_framework_plugin::LocalizedLabel::native("Grid", "Raster"));
    let preview_window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("preview pane");
    assert_eq!(preview_window.label, semio_framework_plugin::LocalizedLabel::native("Preview", "Vorschau"));
}

#[test]
fn the_grid_pane_declares_every_mutation_action_as_a_migrated_verb() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    for id in crate::mutations::KINDS {
        assert!(window.actions.iter().any(|action| &action.id == id), "{id} is not clickable in the grid pane");
    }
    for action in &window.actions {
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "{} would be hard-dead in the interactive app", action.id);
    }
}

#[test]
fn the_preview_pane_owns_the_solve_verb() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("preview pane");
    assert!(window.actions.iter().any(|action| action.id == "solve"));
}

#[test]
fn the_grid_pane_arms_exactly_the_three_declared_utilities() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    let armed: Vec<&str> = window.utilities.iter().map(semio_framework_plugin::UtilityRef::as_str).collect();
    for utility in [grid::UTILITY_SELECT, grid::UTILITY_PIN, grid::UTILITY_MASK] {
        assert!(armed.contains(&utility), "{utility} is not armed: {armed:?}");
    }
    assert_eq!(edit::utilities().len(), 3);
}

/// 🎮️ Every verb THIS editor authors resolves to a typed command. Framework-reserved rows the
/// builder injects (history/clipboard/revert/filter) are deliberately excluded: they are dispatched
/// by the shell, never through an app's own command channel.
#[test]
fn every_authored_action_id_maps_back_to_a_typed_command() {
    const AUTHORED: &[&str] = &["pick-cell", "set-active-tile", "set-camera", "set-grid-visible", "set-grid-snap-enabled", "set-grid-factor", "solve"];
    let definition = create_grid2d_editor();
    let declared: Vec<String> = definition.window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone())).collect();
    for id in crate::mutations::KINDS.iter().copied().chain(AUTHORED.iter().copied()) {
        assert!(declared.iter().any(|declared| declared == id), "{id} is declared by no pane");
        <Grid2dEditor as ArtifactEditor>::command_from_action(id, None).unwrap_or_else(|error| panic!("{id}: {error:?}"));
    }
}

#[test]
fn an_unknown_action_is_refused_rather_than_silently_defaulted() {
    assert!(<Grid2dEditor as ArtifactEditor>::command_from_action("not-a-verb", None).is_err());
}

#[test]
fn the_command_channel_round_trips_through_its_binary_op_form() {
    for command in [
        Grid2dEditorCommand::ChangeSeed { seed: 99 },
        Grid2dEditorCommand::PickCell { x: 2, y: 1 },
        Grid2dEditorCommand::SetCamera { x: 1.0, y: 2.0, zoom: 3.0 },
        Grid2dEditorCommand::Solve,
    ] {
        let bytes = <Grid2dEditorCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
        assert_eq!(<Grid2dEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode"), command);
    }
}

#[test]
fn the_boot_document_is_the_first_bundled_example() {
    assert_eq!(<Grid2dEditor as ArtifactEditor>::initial_snapshot(), crate::examples::grid2d::pipes::document());
}

#[test]
fn the_armed_utility_falls_back_through_window_then_focus_then_flat_then_select() {
    let mut view = ViewModel::default();
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
    view.active_utility_id = Some(grid::UTILITY_MASK.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_MASK);
    view.focused_window_id = Some("focused".into());
    view.active_utility_by_window_id.insert("focused".into(), grid::UTILITY_PIN.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_PIN);
    view.window_id = Some("rendered".into());
    view.active_utility_by_window_id.insert("rendered".into(), grid::UTILITY_SELECT.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
}
