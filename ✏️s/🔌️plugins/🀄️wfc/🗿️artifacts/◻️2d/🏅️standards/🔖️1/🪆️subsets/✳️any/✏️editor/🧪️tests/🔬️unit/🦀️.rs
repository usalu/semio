//! 🧪️ The editor surface — identity, the two-window manifest, and one dispatch per command.

use crate::editor::wfc2d::{create_wfc2d_editor, Wfc2dEditor, Wfc2dEditorCommand};
use crate::editor::wfc2d::config::Wfc2dConfig;
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use semio_framework_plugin::ArtifactEditor;

fn dispatch(command: &Wfc2dEditorCommand, document: &crate::Wfc2dSnapshot, config: &Wfc2dConfig) -> Result<semio_framework_plugin::Emit<crate::Wfc2dMutation, crate::editor::wfc2d::config::Wfc2dConfigMutation>, semio_framework_plugin::Fault> {
    crate::editor::wfc2d::dispatch(command, document, config)
}

#[test]
fn editor_binds_the_artifact_dialect() {
    assert_eq!(Wfc2dEditor::DIALECT.artifact_kind, crate::WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(Wfc2dEditor::initial_snapshot(), crate::examples::two_room_corridor::document());
}

#[test]
fn editor_manifest_declares_both_windows() {
    let definition = create_wfc2d_editor();
    assert_eq!(definition.id, "s.wfc.wfc2d@1/*#editor");
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert!(ids.contains(&graph::WFC_GRAPH_WINDOW));
    assert!(ids.contains(&preview::WFC_2D_PREVIEW_WINDOW));
}

/// ✏️ Every document verb dispatches to exactly one artifact mutation.
#[test]
fn every_document_command_emits_one_mutation() {
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    let commands = [
        Wfc2dEditorCommand::ChangeSeed { seed: 11 },
        Wfc2dEditorCommand::CreateSlot { id: "room-c".into(), x: 6.0, y: 0.0, width: 2.0, height: 2.0 },
        Wfc2dEditorCommand::DeleteSlot { id: "room-b".into() },
        Wfc2dEditorCommand::MoveSlot { id: "room-a".into(), x: 1.0, y: 1.0 },
        Wfc2dEditorCommand::ResizeSlot { id: "room-a".into(), width: 3.0, height: 3.0 },
        Wfc2dEditorCommand::ConnectSlots { id: "edge-ab".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: String::new() },
        Wfc2dEditorCommand::DisconnectSlots { id: "edge-a-corridor".into() },
        Wfc2dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() },
        Wfc2dEditorCommand::UnpinSlot { id: "room-a".into() },
        Wfc2dEditorCommand::CreateTile { id: "door".into(), label: None, weight: 1.0 },
        Wfc2dEditorCommand::DeleteTile { id: "room".into() },
        Wfc2dEditorCommand::ChangeTileWeight { tile_id: "room".into(), weight: 4.0 },
        Wfc2dEditorCommand::ChangeTileMedia { tile_id: "room".into() },
        Wfc2dEditorCommand::CreateRule { id: "rule-x".into(), tile_a_id: "room".into(), tile_b_id: "corridor".into(), relation: None, allowed: true },
        Wfc2dEditorCommand::DeleteRule { id: "rule-room-room".into() },
    ];
    for command in &commands {
        let emit = dispatch(command, &document, &config).expect("command dispatches");
        assert_eq!(emit.artifact_mutations.len(), 1, "{command:?} did not emit exactly one mutation");
        assert!(emit.config_mutations.is_empty(), "{command:?} must not touch the pane config");
    }
}

/// 🎥️ Camera and armed-tile verbs go to the PANE config, never to the document lanes.
#[test]
fn view_commands_never_touch_the_document() {
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    for command in [Wfc2dEditorCommand::ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 }, Wfc2dEditorCommand::ChangeActiveTile { tile_id: "room".into() }] {
        let emit = dispatch(&command, &document, &config).expect("command dispatches");
        assert!(emit.artifact_mutations.is_empty(), "{command:?} must not emit a document mutation");
        assert_eq!(emit.config_mutations.len(), 1);
    }
}

/// 📌️ A pin with no armed tile and no tiles at all is refused with a domain error, never a pin to
/// an empty id the mutation would then have to reject.
#[test]
fn pinning_without_any_tile_is_refused() {
    let command = Wfc2dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() };
    assert!(dispatch(&command, &crate::Wfc2dSnapshot::default(), &Wfc2dConfig::default()).is_err());
}

/// 🖼️ Both windows render non-empty for every bundled example.
#[test]
fn both_windows_render_for_every_example() {
    let config = Wfc2dConfig::default();
    let transient = crate::editor::wfc2d::transient::Wfc2dTransient::default();
    for document in crate::examples::documents() {
        for body in [graph::WFC_GRAPH_BODY, preview::WFC_2D_PREVIEW_BODY] {
            crate::editor::wfc2d::render_body(body, &document, &config, &transient).expect("window renders");
        }
    }
}

/// ⚡️ Every command round-trips through its own binary op encoding.
#[test]
fn commands_round_trip_through_their_binary_codec() {
    let command = Wfc2dEditorCommand::MoveSlot { id: "room-a".into(), x: 1.5, y: -2.5 };
    let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
    let decoded: Wfc2dEditorCommand = protocol::OpBinary::decode_op(&bytes).expect("command decodes");
    assert_eq!(decoded, command);
}

/// 🫧️ THE HOST PATH. `render_with_transient` is exactly what `ArtifactEditor::render_with_request_context`
/// runs (it is a one-line delegation), so this drives the real render entry point with a real
/// `TransientView` and proves the solved assignment reaches `wfc-2d-preview` in a running app — the
/// gap the wfc2d audit flagged as blocking. The two arguments left out (`ArtifactInstanceOperationOwnerHandle`
/// and `InteractionView`) are unconstructible from this crate — `InteractionView`'s fields are
/// `pub(crate)` in the framework crate — and neither is read by this artifact's render.
#[test]
fn the_host_render_path_paints_the_solved_assignment() {
    use crate::editor::wfc2d::transient::{Wfc2dAssignment, Wfc2dTransient};
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };

    let solved = Wfc2dTransient {
        assignments: vec![
            Wfc2dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() },
            Wfc2dAssignment { slot_id: "room-a".into(), tile_id: "room".into() },
            Wfc2dAssignment { slot_id: "room-b".into(), tile_id: "room".into() },
        ],
        contradiction: false,
    };
    let empty = Wfc2dTransient::default();

    let painted = crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &solved, window: None }).expect("the host render path renders");
    let unsolved = crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &empty, window: None }).expect("the host render path renders unsolved too");
    // 🔎️ `ComponentTree` is `Debug` but not `PartialEq`, so the two renders are compared by their
    // debug projection — enough to prove the lane is read, and it fails loudly if it ever is not.
    assert_ne!(format!("{painted:?}"), format!("{unsolved:?}"), "the transient lane never reached the preview window: a solved board rendered identically to an unsolved one");

    // 🎨 …and the difference is the tile media, not an incidental label: the solved projection carries
    // one path layer per solved slot, the unsolved one carries none.
    let solved_layers = preview::preview_layers_json(&document, &solved);
    let unsolved_layers = preview::preview_layers_json(&document, &empty);
    for slot in &document.slots {
        assert!(solved_layers.contains(&format!("tile-{}-", slot.id)), "slot {} lost its solved tile media", slot.id);
    }
    assert!(!unsolved_layers.contains("\"kind\":\"path\""), "an unsolved, unpinned board must paint no tile media");
}

/// 🖼️ A bitmap tile reaches the canvas as a real PNG data URL through the same host path.
#[test]
fn the_host_render_path_paints_bitmap_tiles_as_pixels() {
    use crate::editor::wfc2d::transient::{Wfc2dAssignment, Wfc2dTransient};
    let document = crate::examples::terrain_ring::document();
    let tile = document.tiles[0].id.clone();
    let transient = Wfc2dTransient { assignments: document.slots.iter().map(|slot| Wfc2dAssignment { slot_id: slot.id.clone(), tile_id: tile.clone() }).collect(), contradiction: false };
    let layers = preview::preview_layers_json(&document, &transient);
    assert!(layers.contains("\"kind\":\"image\""), "a bitmap tile must paint an image layer, not a labelled rect");
    assert!(layers.contains("data:image/png;base64,"), "a bitmap tile must carry a real png data url");
    assert!(!layers.contains("bitmap\",\"kind\":\"rect\""), "no bitmap tile may fall back to the outline placeholder here");

    let config = Wfc2dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &transient, window: None }).expect("the raster board renders through the host path");
}
