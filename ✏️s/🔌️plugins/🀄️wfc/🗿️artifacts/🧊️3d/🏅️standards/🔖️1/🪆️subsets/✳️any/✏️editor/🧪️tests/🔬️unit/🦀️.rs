//! 🧪️ Editor surface laws: the manifest is canonical, every command dispatches to the mutation it
//! names, camera and armed-tile verbs never touch the document, and both windows render.

use super::*;
use semio_framework_plugin::ArtifactEditor;

fn document() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

/// ✏️ `ArtifactView`/`ConfigView` carry private fields, so nothing outside the framework can build
/// one — the editor's own pure `command_emit`/`render_body` split is what makes the mapping and the
/// per-window render reachable from here, and `handle`/`render` are one-line adapters over them.
fn dispatch(command: &Wfc3dEditorCommand, snapshot: &Wfc3dSnapshot, config: &Wfc3dConfig) -> Result<Emit<Wfc3dMutation, Wfc3dConfigMutation>, Fault> {
    command_emit(command, snapshot, config)
}

#[test]
fn the_editor_app_id_is_the_canonical_surface_id() {
    let definition = create_wfc3d_editor();
    assert_eq!(definition.id, "s.wfc.wfc3d@1/*#editor");
    assert_eq!(definition.breadcrumb, vec!["semio".to_string(), "wfc".to_string(), "3d".to_string()]);
}

/// 🪟️ Two windows, one layout: the problem on the left, its solution on the right.
#[test]
fn the_editor_declares_the_graph_and_the_preview_window() {
    let definition = create_wfc3d_editor();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert_eq!(ids, vec![graph::WFC_GRAPH_WINDOW, preview::WFC_3D_PREVIEW_WINDOW]);
}

#[test]
fn the_editor_boots_on_a_real_document() {
    assert_eq!(<Wfc3dEditor as ArtifactEditor>::initial_snapshot(), document());
    assert_eq!(<Wfc3dEditor as ArtifactEditor>::DIALECT, WFC3D_DIALECT);
}

/// ✏️ One command, one mutation — every user-facing verb reaches the schema tree's own builder.
#[test]
fn every_document_command_dispatches_to_exactly_one_mutation() {
    let document = document();
    let config = Wfc3dConfig::default();
    let commands = vec![
        Wfc3dEditorCommand::ChangeSeed { seed: 11 },
        Wfc3dEditorCommand::CreateSlot { id: "room-c".into(), x: 3.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0 },
        Wfc3dEditorCommand::DeleteSlot { id: "room-b".into() },
        Wfc3dEditorCommand::MoveSlot { id: "room-b".into(), x: 2.0, y: 1.0, z: 0.0 },
        Wfc3dEditorCommand::ResizeSlot { id: "room-a".into(), width: 2.0, height: 1.0, depth: 1.0 },
        Wfc3dEditorCommand::ConnectSlots { id: "edge-a-b".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: String::new() },
        Wfc3dEditorCommand::DisconnectSlots { id: "edge-corridor-b".into() },
        Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() },
        Wfc3dEditorCommand::UnpinSlot { id: "room-a".into() },
        Wfc3dEditorCommand::CreateTile { id: "stair".into(), label: None, weight: 0.0 },
        Wfc3dEditorCommand::DeleteTile { id: "corridor".into() },
        Wfc3dEditorCommand::ChangeTileWeight { tile_id: "room".into(), weight: 4.0 },
        Wfc3dEditorCommand::ChangeTileMedia { tile_id: "room".into() },
        Wfc3dEditorCommand::CreateRule { id: "rule-x".into(), tile_a_id: "room".into(), tile_b_id: "corridor".into(), relation: None, allowed: false },
        Wfc3dEditorCommand::DeleteRule { id: "rule-room-room".into() },
    ];
    assert_eq!(commands.len(), crate::mutations::KINDS.len(), "every document mutation kind has a command");
    for command in commands {
        let emit = match dispatch(&command, &document, &config) {
            Ok(emit) => emit,
            Err(fault) => panic!("{command:?} must dispatch: {fault:?}"),
        };
        assert_eq!(emit.artifact_mutations.len(), 1, "{command:?} must emit exactly one mutation");
        assert!(emit.config_mutations.is_empty(), "{command:?} must not touch the pane config");
        assert!(emit.description.is_some());
    }
}

/// 🎥️ The two view verbs go to the PER-PANE config and never to the document, so a camera move can
/// never enter the undo history.
#[test]
fn the_view_verbs_touch_the_config_and_never_the_document() {
    let document = document();
    let config = Wfc3dConfig::default();
    for command in [Wfc3dEditorCommand::ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 }, Wfc3dEditorCommand::ChangeActiveTile { tile_id: "room".into() }] {
        let emit = match dispatch(&command, &document, &config) {
            Ok(emit) => emit,
            Err(fault) => panic!("{command:?} must dispatch: {fault:?}"),
        };
        assert!(emit.artifact_mutations.is_empty(), "{command:?} must author no document mutation");
        assert_eq!(emit.config_mutations.len(), 1);
    }
}

/// 🀄️ A pin with no explicit tile uses the PANE's armed tile; with none armed it falls back to the
/// document's first tile rather than emitting an id the mutation would refuse.
#[test]
fn a_pin_with_no_explicit_tile_uses_the_panes_armed_tile() {
    let document = document();
    let armed = Wfc3dConfig { active_tile_id: "room".into(), ..Default::default() };
    let emit = dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &armed).unwrap_or_else(|_| panic!("pin dispatches"));
    assert_eq!(emit.description.as_deref(), Some("Pin room-a to room"));
    let emit = dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("pin dispatches"));
    assert_eq!(emit.description.as_deref(), Some("Pin room-a to corridor"), "the fallback is the document's FIRST tile");
}

#[test]
fn a_pin_into_an_empty_catalogue_is_a_clear_domain_fault_not_a_refused_mutation() {
    let mut document = document();
    document.tiles.clear();
    document.rules.clear();
    let fault = match dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &Wfc3dConfig::default()) {
        Ok(_) => panic!("a pin into an empty catalogue must fault"),
        Err(fault) => fault,
    };
    assert!(format!("{fault:?}").contains("wfc3d.tile.unknown-pin"));
}

/// 📐️ A zero extent from a form default is substituted with the unit value, so the editor never
/// emits a command its own invariant guard would refuse.
#[test]
fn a_zero_extent_command_is_normalised_to_a_unit_box() {
    let document = document();
    let emit = dispatch(&Wfc3dEditorCommand::CreateSlot { id: "room-c".into(), x: 0.0, y: 0.0, z: 0.0, width: 0.0, height: 0.0, depth: 0.0 }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("dispatch"));
    let (applied, _) = vcs::apply_mutation(&document, &emit.artifact_mutations[0]).expect("the normalised slot applies");
    let slot = applied.slots.iter().find(|slot| slot.id == "room-c").expect("room-c exists");
    assert_eq!((slot.width, slot.height, slot.depth), (1.0, 1.0, 1.0));
}

/// 🔤️ A created row lands at its collection's canonical sorted position, so an undo restores it there.
#[test]
fn a_created_row_lands_at_its_canonical_sorted_position() {
    let document = document();
    let emit = dispatch(&Wfc3dEditorCommand::CreateTile { id: "attic".into(), label: None, weight: 1.0 }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("dispatch"));
    let (applied, _) = vcs::apply_mutation(&document, &emit.artifact_mutations[0]).expect("apply");
    assert_eq!(applied.tiles.first().map(|tile| tile.id.as_str()), Some("attic"), "\"attic\" sorts before \"corridor\"");
}

#[test]
fn both_window_bodies_render_for_every_example() {
    let config = Wfc3dConfig::default();
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        for body in [graph::WFC_GRAPH_BODY, preview::WFC_3D_PREVIEW_BODY] {
            let tree = render_body(body, &document, &config).unwrap_or_else(|error| panic!("{body} must render: {error:?}"));
            assert!(!format!("{tree:?}").is_empty());
        }
    }
}

#[test]
fn an_unknown_body_key_renders_a_label_instead_of_failing() {
    assert!(render_body("nope", &document(), &Wfc3dConfig::default()).is_ok());
}

/// 🕸️ The graph view drops `z` deliberately: the canvas is a plan of an arbitrary graph, and the
/// third axis belongs to the preview pane.
#[test]
fn the_graph_view_projects_x_and_y_and_drops_z() {
    use crate::editor::wfc3d::modes::edit::windows::graph::SlotGraphView;
    let document = crate::examples::tower_stack::snapshot();
    let slots = Wfc3dGraphView(&document).graph_slots();
    assert_eq!(slots.len(), document.slots.len());
    let cantilever = slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever projects");
    assert_eq!((cantilever.x, cantilever.y), (1.5, 3.0));
}
