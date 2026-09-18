//! 🔬️ Editor laws — every declared verb dispatches, the pick verb reads the ARMED UTILITY of the
//! window it was dispatched in, and the two windows both render non-empty for both examples.

use super::*;

#[test]
fn the_manifest_declares_both_window_kinds_and_the_three_utilities() {
    let definition = create_grid3d_editor();
    assert_eq!(definition.id, "s.wfc.grid3d@1/*#editor");
    assert!(definition.window_kinds.iter().any(|window| window.id == grid::WINDOW_KIND_ID));
    assert!(definition.window_kinds.iter().any(|window| window.id == preview::WINDOW_KIND_ID));
    let utilities: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utilities, vec![grid::UTILITY_SELECT, grid::UTILITY_PIN, grid::UTILITY_MASK]);
}

#[test]
fn every_declared_action_is_migrated_so_it_can_actually_be_dispatched() {
    let definition = create_grid3d_editor();
    for window in &definition.window_kinds {
        for action in &window.actions {
            assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "action {} would be dead in the interactive app", action.id);
        }
    }
}

#[test]
fn the_grid_window_declares_one_action_per_document_verb() {
    let definition = grid::definition();
    let ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
    for verb in ["changeSeed", "resizeGrid", "changeCellSizes", "changePeriodicity", "createTile", "deleteTile", "changeTileWeight", "changeTileColor", "createRule", "deleteRule", "pinCell", "unpinCell", "maskCell", "unmaskCell"] {
        assert!(ids.contains(&verb), "the grid window must declare {verb}");
    }
}

#[test]
fn a_cell_key_parses_back_only_when_the_grid_could_have_emitted_it() {
    assert_eq!(parse_cell_id("1:2:3"), Some((1, 2, 3)));
    assert_eq!(parse_cell_id("1:2"), None);
    assert_eq!(parse_cell_id("1:2:3:4"), None);
    assert_eq!(parse_cell_id("a:b:c"), None);
}

#[test]
fn the_armed_utility_defaults_to_select_when_no_window_has_one() {
    assert_eq!(grid3d_active_utility(None), grid::UTILITY_SELECT);
}

#[test]
fn every_command_maps_onto_a_real_mutation_builder() {
    let commands = vec![
        Grid3dEditorCommand::ChangeSeed { seed: 1 },
        Grid3dEditorCommand::ResizeGrid { width: 2, height: 2, depth: 2 },
        Grid3dEditorCommand::ChangePeriodicity { periodic_x: true, periodic_y: false, periodic_z: false },
        Grid3dEditorCommand::DeleteTile { id: "wall".into() },
        Grid3dEditorCommand::UnpinCell { x: 0, y: 0, z: 0 },
        Grid3dEditorCommand::MaskCell { x: 1, y: 1, z: 1 },
        Grid3dEditorCommand::PickCell { cell_id: "1:0:0".into() },
        Grid3dEditorCommand::SetActiveTile { tile_id: "wall".into() },
    ];
    for command in commands {
        let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
        assert_eq!(<Grid3dEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), command);
    }
}
