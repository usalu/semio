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
        Wfc3dEditorCommand::MoveSlot { id: "room-b".into(), x: 2.0, y: 1.0, z: Some(0.0) },
        Wfc3dEditorCommand::ResizeSlot { id: "room-a".into(), width: 2.0, height: 1.0, depth: Some(1.0) },
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
    let transient = crate::editor::wfc3d::transient::Wfc3dTransient::default();
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        for body in [graph::WFC_GRAPH_BODY, preview::WFC_3D_PREVIEW_BODY] {
            let tree = render_body(body, &document, &config, &transient, None).unwrap_or_else(|error| panic!("{body} must render: {error:?}"));
            assert!(!format!("{tree:?}").is_empty());
        }
    }
}

#[test]
fn an_unknown_body_key_renders_a_label_instead_of_failing() {
    assert!(render_body("nope", &document(), &Wfc3dConfig::default(), &crate::editor::wfc3d::transient::Wfc3dTransient::default(), None).is_ok());
}

/// 🕸️ The graph view drops `z` deliberately: the canvas is a plan of an arbitrary graph, and the
/// third axis belongs to the preview pane. It also SCALES into canvas units, because a slot is a box
/// in metres and a one-metre node at viewport zoom 1 is one pixel.
#[test]
fn the_graph_view_projects_x_and_y_scaled_and_drops_z() {
    use crate::editor::wfc3d::modes::edit::windows::graph::SlotGraphView;
    let document = crate::examples::tower_stack::snapshot();
    let slots = Wfc3dGraphView(&document).graph_slots();
    assert_eq!(slots.len(), document.slots.len());
    let cantilever = slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever projects");
    assert_eq!((cantilever.x, cantilever.y), (1.5 * WFC_3D_GRAPH_UNIT, 3.0 * WFC_3D_GRAPH_UNIT));
    assert_eq!((slot_coordinate(cantilever.x), slot_coordinate(cantilever.y)), (1.5, 3.0), "the canvas→document inverse is exact, or a released drag lands a wrong move-slot");
}

/// 🚚️ A released node drag is ONE `move-slot` in DOCUMENT units that keeps the slot's authored `z` —
/// the canvas never saw the third axis, so reading it back off the document is the only thing that
/// stops a drag flattening a stack.
#[test]
fn a_dragged_node_lands_one_move_slot_in_document_units_keeping_z() {
    let document = crate::examples::tower_stack::snapshot();
    let authored = document.slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever exists").clone();
    let operations = format!(r#"[{{"operation":"move","nodeId":"cantilever","x":{},"y":{}}}]"#, 4.0 * WFC_3D_GRAPH_UNIT, 5.0 * WFC_3D_GRAPH_UNIT);
    let (mutations, description) = graph_edit_mutations(&document, &operations).expect("the gesture lowers");
    assert_eq!(mutations.len(), 1, "one gesture is one edit, never one per pointer tick");
    assert_eq!(description, "Move slot cantilever");
    assert_eq!(mutations, vec![move_slot("cantilever".into(), 4.0, 5.0, authored.z)]);
}

/// 🔗 A connect gesture between two slot nodes mints ONE deterministic edge; repeating it is a
/// no-op, because a duplicate edge id is a fatal mutation invariant, not a second edge.
#[test]
fn a_connect_gesture_lands_one_connect_slots_and_never_duplicates() {
    let document = crate::examples::two_room_corridor::snapshot();
    let operations = r#"[{"operation":"connect","sourceNodeId":"room-a","sourcePortId":"room-a@adjacent-out","targetNodeId":"room-b","targetPortId":"room-b@adjacent-in"}]"#;
    let (mutations, description) = graph_edit_mutations(&document, operations).expect("the gesture lowers");
    assert_eq!(mutations.len(), 1);
    assert_eq!(description, "Connect room-a to room-b");
    let repeated = r#"[{"operation":"connect","sourceNodeId":"room-a","targetNodeId":"corridor"}]"#;
    assert!(graph_edit_mutations(&document, repeated).expect("the gesture lowers").0.is_empty(), "room-a already borders the corridor, so the gesture authors nothing");
}

/// 🎨️ Every bundled example resolves by its registered id, the empty id is the shell's own "default
/// document", and an unregistered id faults instead of opening a blank document.
#[test]
fn set_active_example_resolves_every_registered_example_and_refuses_the_rest() {
    for id in WFC_3D_EXAMPLE_IDS {
        assert_eq!(example_snapshot(id).expect("a registered example loads").schema, WFC3D_DOCUMENT_SCHEMA);
    }
    assert_eq!(example_snapshot("").expect("the empty id is the boot document"), crate::examples::two_room_corridor::snapshot());
    assert!(example_snapshot("nope").is_err(), "an unregistered example id must fault, not load a blank document");
}

/// 🎯️ Every action the editor manifest declares must bridge through `command_from_action`, or the
/// shell's Actions pane, its example picker and the canvas gestures all answer `dispatch-failed`.
#[test]
fn every_declared_action_bridges_through_command_from_action() {
    let definition = create_wfc3d_editor();
    let skip = [
        "undo",
        "redo",
        "commitCheckpoint",
        "createAlternative",
        "switchAlternative",
        "checkoutCheckpoint",
        "copy",
        "cut",
        "paste",
        "revertToCommand",
        "setHistoryCommandFilter",
        "noteShellCommand",
        "recordTutorial",
        "startIntroduction",
        "startTutorial",
        "setActiveUtility",
        "setActiveTool",
        "interactionSelect",
        "interactionHover",
        "clearSelection",
        "selectAll",
        "setSelectionMode",
        "setInteractionGranularity",
        "toolRunStart",
        "toolRunPause",
        "toolRunResume",
        "toolRunStep",
        "toolRunAbort",
        "toolRunFinalize",
        "toolRunDismiss",
        "commit-fill",
    ];
    let mut bridged = 0;
    for window in &definition.window_kinds {
        for action in &window.actions {
            if skip.contains(&action.id.as_str()) {
                continue;
            }
            <Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::command_from_action(&action.id, None).unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
            bridged += 1;
        }
    }
    assert!(bridged >= 10, "expected every declared wfc3d verb to bridge, saw {bridged}");
}

/// 🕸️ The wasm node-graph surface commits a released gesture as a WHOLE graph
/// (`setHostSnapshot`), not as a `move`/`connect` pair. A drag must therefore still land exactly one
/// `move-slot`, in document units, with `z` kept — and an untouched graph must land NOTHING, or every
/// click in the pane would mint an edit.
#[test]
fn a_host_snapshot_commit_lands_only_what_actually_changed() {
    let document = crate::examples::two_room_corridor::snapshot();
    let unchanged = serde_json::json!({
        "operation": "setHostSnapshot",
        "hostSnapshotJson": serde_json::to_string(&serde_json::json!({
            "nodes": document.slots.iter().map(|slot| serde_json::json!({ "id": slot.id, "x": slot.x * WFC_3D_GRAPH_UNIT, "y": slot.y * WFC_3D_GRAPH_UNIT })).collect::<Vec<_>>(),
            "edges": document.edges.iter().map(|edge| serde_json::json!({ "id": edge.id, "source": edge.from_slot_id, "target": edge.to_slot_id })).collect::<Vec<_>>(),
        })).expect("host snapshot"),
    });
    assert!(graph_edit_mutations(&document, &serde_json::Value::Array(vec![unchanged]).to_string()).expect("lowers").0.is_empty(), "an untouched graph owes no edit");

    let room_a = document.slots.iter().find(|slot| slot.id == "room-a").expect("room-a").clone();
    let dragged = serde_json::json!({
        "operation": "setHostSnapshot",
        "hostSnapshotJson": serde_json::to_string(&serde_json::json!({
            "nodes": document.slots.iter().map(|slot| {
                let y = if slot.id == "room-a" { 2.0 } else { slot.y };
                serde_json::json!({ "id": slot.id, "x": slot.x * WFC_3D_GRAPH_UNIT, "y": y * WFC_3D_GRAPH_UNIT })
            }).collect::<Vec<_>>(),
            "edges": document.edges.iter().map(|edge| serde_json::json!({ "id": edge.id, "source": format!("{}@adjacent-out", edge.from_slot_id), "target": format!("{}@adjacent-in", edge.to_slot_id) })).collect::<Vec<_>>(),
        })).expect("host snapshot"),
    });
    let (mutations, description) = graph_edit_mutations(&document, &serde_json::Value::Array(vec![dragged]).to_string()).expect("lowers");
    assert_eq!(mutations, vec![move_slot("room-a".into(), room_a.x, 2.0, room_a.z)]);
    assert_eq!(description, "Move slot room-a");
}

/// 🔗 A wire drawn on the wasm surface arrives as a NEW edge inside the whole-graph commit, with
/// `{node}@{port}` endpoints — one `connect-slots`, endpoints resolved back to their slots.
#[test]
fn a_host_snapshot_commit_lands_a_new_wire_as_connect_slots() {
    let document = crate::examples::two_room_corridor::snapshot();
    let mut edges: Vec<serde_json::Value> = document
        .edges
        .iter()
        .map(|edge| serde_json::json!({ "id": edge.id, "source": format!("{}@adjacent-out", edge.from_slot_id), "target": format!("{}@adjacent-in", edge.to_slot_id) }))
        .collect();
    edges.push(serde_json::json!({ "id": "wire-1", "source": "room-a@adjacent-out", "target": "room-b@adjacent-in" }));
    let wired = serde_json::json!({
        "operation": "setHostSnapshot",
        "hostSnapshotJson": serde_json::to_string(&serde_json::json!({
            "nodes": document.slots.iter().map(|slot| serde_json::json!({ "id": slot.id, "x": slot.x * WFC_3D_GRAPH_UNIT, "y": slot.y * WFC_3D_GRAPH_UNIT })).collect::<Vec<_>>(),
            "edges": edges,
        })).expect("host snapshot"),
    });
    let (mutations, description) = graph_edit_mutations(&document, &serde_json::Value::Array(vec![wired]).to_string()).expect("lowers");
    assert_eq!(mutations.len(), 1);
    assert_eq!(description, "Connect room-a to room-b");
}

/// ✂️ The canvas mints its OWN wire ids, so a whole-graph commit that re-ids the authored edges must
/// not read them as deletions. Only an adjacency the commit genuinely dropped is a `disconnect-slots`,
/// and the pair is unordered.
#[test]
fn a_host_snapshot_commit_never_disconnects_an_edge_it_only_renamed() {
    let document = crate::examples::two_room_corridor::snapshot();
    let renamed: Vec<serde_json::Value> = document
        .edges
        .iter()
        .enumerate()
        .map(|(index, edge)| serde_json::json!({ "id": format!("wire-{index}"), "source": format!("{}@adjacent-out", edge.to_slot_id), "target": format!("{}@adjacent-in", edge.from_slot_id) }))
        .collect();
    let commit = serde_json::json!({
        "operation": "setHostSnapshot",
        "hostSnapshotJson": serde_json::to_string(&serde_json::json!({
            "nodes": document.slots.iter().map(|slot| serde_json::json!({ "id": slot.id, "x": slot.x * WFC_3D_GRAPH_UNIT, "y": slot.y * WFC_3D_GRAPH_UNIT })).collect::<Vec<_>>(),
            "edges": renamed,
        })).expect("host snapshot"),
    });
    assert!(
        graph_edit_mutations(&document, &serde_json::Value::Array(vec![commit]).to_string()).expect("lowers").0.is_empty(),
        "re-ided and reversed edges are the same adjacencies, so the commit owes nothing"
    );

    let dropped = serde_json::json!({
        "operation": "setHostSnapshot",
        "hostSnapshotJson": serde_json::to_string(&serde_json::json!({
            "nodes": document.slots.iter().map(|slot| serde_json::json!({ "id": slot.id, "x": slot.x * WFC_3D_GRAPH_UNIT, "y": slot.y * WFC_3D_GRAPH_UNIT })).collect::<Vec<_>>(),
            "edges": [renamed[0].clone()],
        })).expect("host snapshot"),
    });
    let (mutations, description) = graph_edit_mutations(&document, &serde_json::Value::Array(vec![dropped]).to_string()).expect("lowers");
    assert_eq!(mutations.len(), 1, "exactly the one adjacency the commit really dropped");
    assert!(description.starts_with("Disconnect "), "saw {description}");
}


/// 🚚️ The `wfc-graph` window is shared with `wfc2d`, so its staged `move-slot`/`resize-slot` forms
/// offer only `x`/`y` and `width`/`height`. A dispatch that omits the third axis must KEEP the slot's
/// authored `z`/`depth` — a 2d form must not be able to flatten a stack it cannot see.
#[test]
fn a_two_axis_move_or_resize_keeps_the_authored_third_axis() {
    let document = crate::examples::tower_stack::snapshot();
    let authored = document.slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever exists").clone();
    let config = Wfc3dConfig::default();

    let moved = dispatch(&Wfc3dEditorCommand::MoveSlot { id: "cantilever".into(), x: 9.0, y: 9.0, z: None }, &document, &config).expect("move dispatches");
    assert_eq!(moved.artifact_mutations, vec![move_slot("cantilever".into(), 9.0, 9.0, authored.z)]);

    let resized = dispatch(&Wfc3dEditorCommand::ResizeSlot { id: "cantilever".into(), width: 4.0, height: 5.0, depth: None }, &document, &config).expect("resize dispatches");
    assert_eq!(resized.artifact_mutations, vec![resize_slot("cantilever".into(), 4.0, 5.0, authored.depth)]);
}

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
    let tools: std::collections::BTreeSet<&str> = WFC_3D_RETAINED_TOOL_IDS.iter().copied().collect();
    let publication: std::collections::BTreeSet<&str> = <Wfc3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Wfc3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
    }
    let proofs: std::collections::BTreeSet<&str> = <Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
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
    let definition = create_wfc3d_editor();
    let declared: std::collections::BTreeMap<String, semio_framework::InteractiveJobClassification> = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .map(|action| (action.id.clone(), action.semantics.execution.interactive_job))
        .collect();
    for tool_id in WFC_3D_RETAINED_TOOL_IDS {
        assert_eq!(
            declared.get(*tool_id),
            Some(&semio_framework::InteractiveJobClassification::Migrated),
            "retained tool '{tool_id}' is declared by no window kind and no app action, so the framework refuses its proof row"
        );
    }
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a WFC 3D rule set can be created and opened as a hub document.
#[test]
fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_wfc3d_editor().artifact_kinds, vec![crate::artifact_kind()]);
}
