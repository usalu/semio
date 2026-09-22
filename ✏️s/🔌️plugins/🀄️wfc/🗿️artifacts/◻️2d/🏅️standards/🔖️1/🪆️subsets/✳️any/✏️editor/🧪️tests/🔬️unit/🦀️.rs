//! 🧪️ The editor surface — identity, the two-window manifest, and one dispatch per command.

use crate::editor::wfc2d::{create_wfc2d_editor, Wfc2dEditor, Wfc2dEditorCommand};
// 🔒️ The retained factory is private to the editor module, so the glob `pub use component::*`
// re-export does not carry it — the roster law below reaches it through its defining module.
use crate::editor::wfc2d::component::Wfc2dRetainedCommandJobFactory;
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
            crate::editor::wfc2d::render_body(body, &document, &config, &transient, None).expect("window renders");
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
/// 🎨 …and the difference is the tile media, not an incidental label: the solved projection carries
/// one path layer per solved slot, the unsolved one carries none.
/// 🔎️ `ComponentTree` is `Debug` but not `PartialEq`, so the two renders are compared by their
/// debug projection — enough to prove the lane is read, and it fails loudly if it ever is not.
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
    assert_ne!(format!("{painted:?}"), format!("{unsolved:?}"), "the transient lane never reached the preview window: a solved board rendered identically to an unsolved one");

    let solved_layers = preview::preview_layers_json(&document, &solved, None);
    let unsolved_layers = preview::preview_layers_json(&document, &empty, None);
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
    let layers = preview::preview_layers_json(&document, &transient, None);
    assert!(layers.contains("\"kind\":\"image\""), "a bitmap tile must paint an image layer, not a labelled rect");
    assert!(layers.contains("data:image/png;base64,"), "a bitmap tile must carry a real png data url");
    assert!(!layers.contains("bitmap\",\"kind\":\"rect\""), "no bitmap tile may fall back to the outline placeholder here");

    let config = Wfc2dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &transient, window: None }).expect("the raster board renders through the host path");
}

//#region 🕹️GraphGestures
/// 🕹️ Every canvas gesture arrives as ONE `nodeGraphEdit`; the wasm node-graph surface hands back the
/// WHOLE graph, so a drag is read as the node whose position moved.
#[test]
fn a_dragged_node_lands_exactly_one_move_slot() {
    let document = crate::examples::two_room_corridor::document();
    let scale = crate::editor::wfc2d::WFC_2D_GRAPH_VIEW_SCALE;
    let snapshot = format!(
        "{{\"schema\":\"dag.host_snapshot\",\"camera\":{{\"x\":0.0,\"y\":0.0,\"zoom\":1.0}},\"nodes\":[{{\"id\":\"room-a\",\"x\":{},\"y\":{}}},{{\"id\":\"corridor\",\"x\":{},\"y\":0.0}},{{\"id\":\"room-b\",\"x\":{},\"y\":0.0}}],\"edges\":[{{\"id\":\"edge-a-corridor\",\"source\":\"room-a@adjacent-out\",\"target\":\"corridor@adjacent-in\"}},{{\"id\":\"edge-corridor-b\",\"source\":\"corridor@adjacent-out\",\"target\":\"room-b@adjacent-in\"}}]}}",
        7.0 * scale,
        3.0 * scale,
        2.0 * scale,
        4.0 * scale
    );
    let operations = format!("[{{\"operation\":\"setHostSnapshot\",\"hostSnapshotJson\":{}}}]", protocol::json::to_json_string(&snapshot));
    let Ok(emit) = dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: operations }, &document, &Wfc2dConfig::default()) else { panic!("a drag is one edit") };
    assert_eq!(emit.artifact_mutations.len(), 1, "one gesture is one edit");
    assert_eq!(emit.description.as_deref(), Some("Move slot room-a"));
    let mut next = document;
    crate::mutations::apply_wfc2d_mutation(&mut next, &emit.artifact_mutations[0]).expect("the move applies");
    let slot = next.slots.iter().find(|slot| slot.id == "room-a").expect("room-a survives its own move");
    assert!((slot.x - 7.0).abs() < 1e-9 && (slot.y - 3.0).abs() < 1e-9, "the canvas' own units are divided back out: {slot:?}");
}

/// 🔗️ A wire the canvas drew between two slot nodes lands as `connect-slots` with a fresh edge id.
#[test]
fn a_drawn_wire_lands_exactly_one_connect_slots() {
    let document = crate::examples::two_room_corridor::document();
    let snapshot = "{\"schema\":\"dag.host_snapshot\",\"camera\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0},\"nodes\":[],\"edges\":[{\"id\":\"e1\",\"source\":\"room-a@adjacent-out\",\"target\":\"corridor@adjacent-in\"},{\"id\":\"e2\",\"source\":\"corridor@adjacent-out\",\"target\":\"room-b@adjacent-in\"},{\"id\":\"e3\",\"source\":\"room-a@adjacent-out\",\"target\":\"room-b@adjacent-in\"}]}";
    let operations = format!("[{{\"operation\":\"setHostSnapshot\",\"hostSnapshotJson\":{}}}]", protocol::json::to_json_string(&snapshot.to_string()));
    let Ok(emit) = dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: operations }, &document, &Wfc2dConfig::default()) else { panic!("a wire is one edit") };
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert_eq!(emit.description.as_deref(), Some("Connect room-a to room-b"));
}

/// ✂️ A wire the canvas removed lands as `disconnect-slots` naming the document's own edge id.
#[test]
fn a_removed_wire_lands_exactly_one_disconnect_slots() {
    let document = crate::examples::two_room_corridor::document();
    let snapshot = "{\"schema\":\"dag.host_snapshot\",\"camera\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0},\"nodes\":[],\"edges\":[{\"id\":\"e2\",\"source\":\"corridor@adjacent-out\",\"target\":\"room-b@adjacent-in\"}]}";
    let operations = format!("[{{\"operation\":\"setHostSnapshot\",\"hostSnapshotJson\":{}}}]", protocol::json::to_json_string(&snapshot.to_string()));
    let Ok(emit) = dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: operations }, &document, &Wfc2dConfig::default()) else { panic!("a removed wire is one edit") };
    assert_eq!(emit.description.as_deref(), Some("Disconnect edge-a-corridor"));
}

/// 🧘 A gesture that only panned the camera is not an edit at all.
#[test]
fn a_camera_only_gesture_is_no_edit() {
    let document = crate::examples::two_room_corridor::document();
    let scale = crate::editor::wfc2d::WFC_2D_GRAPH_VIEW_SCALE;
    let snapshot = format!(
        "{{\"schema\":\"dag.host_snapshot\",\"camera\":{{\"x\":120.0,\"y\":-40.0,\"zoom\":2.5}},\"nodes\":[{{\"id\":\"room-a\",\"x\":0.0,\"y\":0.0}},{{\"id\":\"corridor\",\"x\":{},\"y\":0.0}},{{\"id\":\"room-b\",\"x\":{},\"y\":0.0}}],\"edges\":[{{\"id\":\"e1\",\"source\":\"room-a@adjacent-out\",\"target\":\"corridor@adjacent-in\"}},{{\"id\":\"e2\",\"source\":\"corridor@adjacent-out\",\"target\":\"room-b@adjacent-in\"}}]}}",
        2.0 * scale,
        4.0 * scale
    );
    let operations = format!("[{{\"operation\":\"setHostSnapshot\",\"hostSnapshotJson\":{}}}]", protocol::json::to_json_string(&snapshot));
    let Ok(emit) = dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: operations }, &document, &Wfc2dConfig::default()) else { panic!("a pan is no edit") };
    assert!(emit.artifact_mutations.is_empty(), "a camera-only gesture must mint nothing");
}

/// 🛂️ A verb aimed at an id the document does not hold is refused BY NAME rather than minting an edit
/// against nothing — the palette's argument defaults are static and outlive the example they were
/// authored against.
#[test]
fn a_verb_against_an_unknown_id_is_refused_by_name() {
    let document = crate::examples::two_room_corridor::document();
    for (command, code) in [
        (Wfc2dEditorCommand::DeleteSlot { id: "nope".into() }, "wfc2d.slot.unknown-slot"),
        (Wfc2dEditorCommand::UnpinSlot { id: "nope".into() }, "wfc2d.slot.unknown-slot"),
        (Wfc2dEditorCommand::DeleteTile { id: "nope".into() }, "wfc2d.tile.unknown-tile"),
        (Wfc2dEditorCommand::DisconnectSlots { id: "nope".into() }, "wfc2d.edge.unknown-edge"),
        (Wfc2dEditorCommand::DeleteRule { id: "nope".into() }, "wfc2d.rule.unknown-rule"),
        (Wfc2dEditorCommand::CreateSlot { id: "room-a".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, "wfc2d.id.taken"),
    ] {
        let Err(fault) = dispatch(&command, &document, &Wfc2dConfig::default()) else { panic!("an unknown id must be refused: {command:?}") };
        assert_eq!(fault.code.0, code, "{command:?}");
    }
}

/// 🏁 The solve the preview paints is the artifact's OWN inference, run to completion.
#[test]
fn the_solve_verb_answers_a_transient_assignment() {
    let document = crate::examples::two_room_corridor::document();
    let mutations = crate::editor::wfc2d::solve_transient(&document).expect("the boot example solves");
    assert_eq!(mutations.len(), 1, "one solve is one transient publication");
    let crate::editor::wfc2d::transient::Wfc2dTransientMutation::SetSolve(solve) = &mutations[0];
    let transient = crate::editor::wfc2d::transient::Wfc2dTransient { assignments: solve.assignments.clone(), contradiction: solve.contradiction };
    assert!(!transient.contradiction, "the boot example is satisfiable");
    assert_eq!(transient.assignments.len(), document.slots.len(), "every slot is assigned");
    for row in &transient.assignments {
        assert!(document.tiles.iter().any(|tile| tile.id == row.tile_id), "slot {} was assigned an undeclared tile", row.slot_id);
    }
}

/// 🗃️ The example picker answers a whole-document LOAD, never a mutation set — which is exactly why
/// re-picking the booted example mints no undo entry.
#[test]
fn the_example_picker_loads_a_document_instead_of_editing_one() {
    let document = crate::examples::two_room_corridor::document();
    for example_id in [crate::examples::two_room_corridor::ID, crate::examples::wall_roof_facade_strip::ID, crate::examples::hex_ring::ID, crate::examples::terrain_ring::ID] {
        let emit = dispatch(&Wfc2dEditorCommand::SetActiveExample { example_id: example_id.to_string() }, &document, &Wfc2dConfig::default()).expect("every offered example loads");
        assert!(emit.artifact_mutations.is_empty(), "an example load is not a document edit");
        assert!(matches!(emit.effects.first(), Some(semio_framework::kernel::Effect::LoadDocument { .. })), "an example load is one LoadDocument effect");
    }
    let Err(fault) = dispatch(&Wfc2dEditorCommand::SetActiveExample { example_id: "not-an-example".into() }, &document, &Wfc2dConfig::default()) else { panic!("an unknown example must be refused") };
    assert_eq!(fault.code.0, "wfc2d.example.unknown");
}
//#endregion 🕹️GraphGestures

/// ⚖️ LAW: this app's one app-owned factory carries ONE roster — `TOOL_IDS`, its
/// `PUBLICATION_CONTRACTS` and its `bounded_first_step_tool_proofs!` rows name exactly the same
/// tools. The framework refuses app registration outright when they drift
/// (`interactive-job.publication-contract` when a lane contract names an unowned tool,
/// `interactive-job.catalog-incomplete` when a migrated command has no owner-local proof), and that
/// refusal is a guest-side `panic!` — so one missing row aborted the whole wfc component at boot and
/// every one of its panes reached `data-shell-error` instead of `data-shell-ready`.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = crate::editor::wfc2d::WFC_2D_RETAINED_TOOL_IDS.iter().copied().collect();
    let publication: std::collections::BTreeSet<&str> = <Wfc2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Wfc2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
    }
    let proofs: std::collections::BTreeSet<&str> = <Wfc2dEditor as semio_framework_plugin::ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}

/// ⚖️ LAW: every retained tool is a DECLARED `Migrated` action of the BUILT manifest. The framework
/// computes `expected = TOOL_JOB_IDS ∩ migrated_tool_ids(definition)` and refuses any
/// `bounded_first_step_tool_proofs!` row outside it with `interactive-job.catalog-authority` — a
/// guest-side `panic!` at app registration that aborts the whole wfc component, so every wfc pane
/// lands on `data-shell-error`. `commit-fill` reached the roster, the lane contracts and the proofs
/// without any window kind or app-level action ever declaring it, which is exactly that refusal.
/// `window_kind_actions` is the same join the live `AppActionRegistry::from_definition` performs
/// (a window's own rows plus the app-level roster no window claims).
#[test]
fn every_retained_tool_is_a_declared_migrated_action_of_the_built_manifest() {
    let definition = create_wfc2d_editor();
    let declared: std::collections::BTreeMap<String, semio_framework::InteractiveJobClassification> = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .map(|action| (action.id.clone(), action.semantics.execution.interactive_job))
        .collect();
    for tool_id in crate::editor::wfc2d::WFC_2D_RETAINED_TOOL_IDS {
        assert_eq!(
            declared.get(*tool_id),
            Some(&semio_framework::InteractiveJobClassification::Migrated),
            "retained tool '{tool_id}' is declared by no window kind and no app action, so the framework refuses its proof row"
        );
    }
}
