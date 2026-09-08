
use super::*;
use crate::editor::sequence::testkit::{new_app, new_app_with_registry_wired};
use semio_framework_plugin::{Locale, PluginApp, Terminology, testkit::assert_undo_redo_round_trip};

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    assert_eq!(default_snapshot().to_fixture().steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = new_app().await;
    assert_undo_redo_round_trip(&mut app, SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("projection").to_fixture().steps.len(), 2, 3).await;
}

/// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
/// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a
/// `MemoryBackbone` converges both sides onto an identical projection.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<SequencePlayApp>, _>(
        "mem://sequence-convergence",
        SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 111.0, y: 0.0 }),
        SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-2".into(), x: 222.0, y: 0.0 }),
        |app| app.snapshot().expect("projection"),
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn sequence_action_ids_resolve_to_labels_in_native_english_and_german() {
    let definition = create_sequence_app();
    for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), label, "{id} action label");
    }
    for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), label, "{id} action label");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = new_app().await;
    assert!(testkit::render(&mut app, "sequence.play.nope").await.contains("Unknown body"));
}

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_sequence_app()).expect("app definition json");
    for id in [main::SEQUENCE_PLAY_WINDOW_MAIN, script::SEQUENCE_PLAY_WINDOW_SCRIPT, compiled::SEQUENCE_PLAY_WINDOW_COMPILED] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::SEQUENCE_PLAY_MODE_EDIT), "edit mode missing from the manifest");
    for body in [SEQUENCE_PLAY_BODY_DOCUMENT, SEQUENCE_PLAY_BODY_CATALOGUE, SEQUENCE_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("computation.sequence"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️ContextMenuTests
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::context_menu`
/// carries no `InteractionView` (a documented framework gap — see `sequence_context_menu_items`'s
/// own doc comment), so this exercises that free function directly with a real `selected` slice
/// instead of going through the app's live (always-empty) `context_menu` trait method.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_nine_rows_and_ends_with_destructive_delete() {
    let registry = AppActionRegistry::from_definition(&create_sequence_app());
    let items = sequence_context_menu_items(&registry, false, None, &["step-1".to_string()]);
    assert!(items.len() <= 9, "expected <= 9 top-level rows, got {} ({items:?})", items.len());
    let last = items.last().expect("at least one row");
    assert_eq!(last.id, "delete-selection");
    assert_eq!(last.destructive, Some(true));
}
//#endregion 🔖️ContextMenuTests

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn sequence_io_declares_steps_in_and_document_ports() {
    let ports = SequencePlayApp::io().expect("io").all_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    assert!(ports.iter().any(|port| port.id == "steps:in"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_steps_in_inserts_a_new_step_from_an_object_payload() {
    let mut app = new_app_with_registry_wired().await;
    let before = app.snapshot().expect("projection").to_fixture().steps.len();
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: json!({ "message": "from upstream" }).to_string() },
    };
    app.import_media("steps:in", media, &semio_framework_plugin::testkit::meta("local")).await.expect("import steps:in");
    let after = app.snapshot().expect("projection").to_fixture();
    assert_eq!(after.steps.len(), before + 1);
    let imported = after.steps.last().expect("imported step");
    assert_eq!(imported.kind, "computation.import");
    assert_eq!(imported.params.get("message").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("from upstream"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_steps_in_wraps_a_bare_scalar_payload() {
    let mut app = new_app_with_registry_wired().await;
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: "42".into() },
    };
    app.import_media("steps:in", media, &semio_framework_plugin::testkit::meta("local")).await.expect("import steps:in");
    let after = app.snapshot().expect("projection").to_fixture();
    let imported = after.steps.last().expect("imported step");
    assert_eq!(imported.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(42.0));
}

#[semio_framework_async_macros::async_test]
async fn import_media_rejects_unknown_port() {
    let mut app = new_app_with_registry_wired().await;
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: "{}".into() },
    };
    assert!(app.import_media("not-a-port", media, &semio_framework_plugin::testkit::meta("local")).await.is_err());
}
//#endregion 🔖️PortTests

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 17, "every SequenceCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// kebab-cased command id, for every row (sequence has no `flow`-style id/keyword divergence).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected: String = id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<SequenceCommand> {
    vec![
        SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 1.0, y: 2.0 }),
        SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot { kind: "log.print".into(), x: 1.0, y: 2.0, owner: "step-1".into(), slot_name: "then".into() }),
        SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) }),
        SequenceCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
        SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 5.0, y: 6.0 }),
        SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() }),
        SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() }),
        SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params_json: "{\"a\":1}".into() }),
        SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: "step-1".into() }),
        SequenceCommand::Reorganize(reorganize::Reorganize {}),
        SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: "topBottom".into() }),
        SequenceCommand::Run(run_command::Run {}),
        SequenceCommand::Stop(stop_command::Stop {}),
        SequenceCommand::SetViewport(set_viewport::SetViewport { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
    ]
}

/// ⚖️ Pinned to the exact hex captured from the pre-merge `sequence_protocol` crate — a
/// regression here is a real wire-format break, not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_row_keeps_its_pre_migration_bytes() {
    let some = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) });
    assert_eq!(protocol::OpText::print_op(&some), "add-step-dropped add-step-dropped kind=log.print x=1 y=2 picked-step-id=step-1");
    assert_eq!(protocol::OpBinary::encode_op(&some).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010202096c6f672e7072696e7406737465702d31040006000105000000000000f03f02050000000000000040030601");
    let none = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: None });
    assert_eq!(protocol::OpText::print_op(&none), "add-step-dropped add-step-dropped kind=log.print x=1 y=2");
    assert_eq!(protocol::OpBinary::encode_op(&none).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010201096c6f672e7072696e74030006000105000000000000f03f02050000000000000040");
}
//#endregion 🔖️CommandSurface

//#region 🔖️HostTests
use neural_engine::Atom;

#[semio_framework_async_macros::async_test]
async fn disconnect_steps_removes_edge() {
    let mut host = SequenceHost::default();
    assert!(host.disconnect_steps("step-1", "step-2"));
    assert!(host.snapshot.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn sync_from_dag_copies_node_positions() {
    let mut host = SequenceHost::default();
    if let Some(node) = host.dag.fixture.nodes.iter_mut().find(|node| node.id == "step-1") {
        node.x = 120.0;
        node.y = 80.0;
    }
    host.sync_from_dag();
    let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").expect("step-1");
    assert_eq!(step.x, 120.0);
    assert_eq!(step.y, 80.0);
}

#[semio_framework_async_macros::async_test]
async fn sync_edges_from_dag_preserves_existing_edge_ids() {
    let mut host = SequenceHost::default();
    let first_id = host.snapshot.edges[0].id.clone();
    host.sync_edges_from_dag();
    assert_eq!(host.snapshot.edges[0].id, first_id);
    host.sync_edges_from_dag();
    assert_eq!(host.snapshot.edges[0].id, first_id);
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_fan_out() {
    let mut host = SequenceHost::default();
    host.snapshot.edges.clear();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    assert!(host.connect_steps("step-1", "step-2").is_ok());
    assert!(host.connect_steps("step-1", "step-3").is_err());
}

#[semio_framework_async_macros::async_test]
async fn build_path_includes_control_bodies() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("key", NeuralValue::Atom(Atom::String("flag".into()))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep {
        id: "step-4".into(),
        kind: "log.print".into(),
        params: StepParams::new().insert("message", NeuralValue::Atom(Atom::String("yes".into()))),
        x: 560.0,
        y: 160.0,
        slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
        collapsed: false,
    });
    host.snapshot.edges.push(SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() });
    let path = host.build_path();
    assert_eq!(path.steps.len(), 3);
    let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
    assert!(control.bodies.contains_key("then"));
    assert_eq!(control.bodies.get("then").map(|body| body.steps.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn rebuild_dag_preserves_selection() {
    let mut host = SequenceHost::default();
    host.dag.set_selection(&["step-1".into()]);
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.rebuild_dag();
    assert!(host.dag.selected_node_ids().contains(&"step-1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn execution_ports_use_triangle_shape() {
    let host = SequenceHost::default();
    let node = host.step_to_dag_node(&host.snapshot.steps[1]);
    assert_eq!(node.inputs()[0].shape, PortShape::Triangle);
    assert_eq!(node.outputs()[0].shape, PortShape::Triangle);
}

#[semio_framework_async_macros::async_test]
async fn function_steps_use_data_ports_without_visible_execution_pins() {
    let host = SequenceHost::default();
    let step = SequenceStep { id: "step-fn".into(), kind: "math.add".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    let node = host.step_to_dag_node(&step);
    assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
}

#[semio_framework_async_macros::async_test]
async fn text_steps_use_data_ports_without_visible_execution_pins() {
    let host = SequenceHost::default();
    let step = SequenceStep { id: "step-txt".into(), kind: "text.concat".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    let node = host.step_to_dag_node(&step);
    assert!(node.inputs().iter().any(|port| port.id == "left" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "into" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
}

#[semio_framework_async_macros::async_test]
async fn replace_snapshot_preserves_next_serial_and_selection() {
    let mut host = SequenceHost::default();
    let first = host.add_step("math.add", 40.0, 40.0);
    host.dag.set_selection(std::slice::from_ref(&first));
    let json = host.to_json().expect("fixture json");
    let round_trip: SequenceFixture = dsl::os_pack::from_json_str(&json).expect("parse");
    host.replace_snapshot(round_trip).expect("replace");
    let second = host.add_step("math.add", 80.0, 80.0);
    assert_ne!(first, second);
    assert!(host.snapshot.steps.iter().any(|step| step.id == first));
    assert!(host.snapshot.steps.iter().any(|step| step.id == second));
    assert!(host.dag.selected_node_ids().contains(&first));
}

#[semio_framework_async_macros::async_test]
async fn repeated_drops_after_replace_snapshot_use_distinct_ids() {
    let mut host = SequenceHost::default();
    let first = host.add_step_dropped("math.add", 10.0, 10.0, None);
    let json = host.to_json().expect("fixture json");
    let round_trip: SequenceFixture = dsl::os_pack::from_json_str(&json).expect("parse");
    host.replace_snapshot(round_trip).expect("replace");
    let second = host.add_step_dropped("math.add", 20.0, 20.0, None);
    assert_ne!(first, second);
    assert_eq!(host.snapshot.steps.iter().filter(|step| step.kind == "math.add").count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_targets_expanded_control_slot() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert_eq!(step.slot.as_ref().map(|slot| slot.name.as_str()), Some("then"));
}

#[semio_framework_async_macros::async_test]
async fn execution_edges_use_sharp_sz_routing() {
    let host = SequenceHost::default();
    let fixture = host.build_dag_fixture();
    assert!(fixture.edges.iter().all(|edge| edge.route_style == EdgeRouteStyle::SharpSz));
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_toggles_control_step() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    assert!(host.set_step_collapsed("step-3", true));
    assert!(host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().collapsed);
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_rejects_unknown_id() {
    let mut host = SequenceHost::default();
    assert!(!host.set_step_collapsed("nope", true));
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_rejects_non_control_step() {
    let mut host = SequenceHost::default();
    assert!(!host.set_step_collapsed("step-1", true));
    assert!(!host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap().collapsed);
}

#[semio_framework_async_macros::async_test]
async fn remove_step_also_removes_slot_children() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    assert!(host.remove_step("step-3"));
    assert!(!host.snapshot.steps.iter().any(|step| step.id == "step-3" || step.id == "step-4"));
}

#[semio_framework_async_macros::async_test]
async fn remove_step_returns_false_for_unknown_id() {
    let mut host = SequenceHost::default();
    assert!(!host.remove_step("nope"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_updates_step_params() {
    let mut host = SequenceHost::default();
    host.set_step_params_json("step-1", r#"{"key":"renamed"}"#).expect("set params");
    let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap();
    assert_eq!(step.params.get("key").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("renamed"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_rejects_unknown_step() {
    let mut host = SequenceHost::default();
    let err = host.set_step_params_json("nope", "{}").unwrap_err();
    assert!(matches!(err, SequenceCoreError::UnknownStep(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_rejects_invalid_json() {
    let mut host = SequenceHost::default();
    let err = host.set_step_params_json("step-1", "not json").unwrap_err();
    assert!(matches!(err, SequenceCoreError::Json(_)));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_self_connect() {
    let mut host = SequenceHost::default();
    assert!(matches!(host.connect_steps("step-1", "step-1").unwrap_err(), SequenceCoreError::SelfConnect));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_unknown_from_step() {
    let mut host = SequenceHost::default();
    assert!(matches!(host.connect_steps("nope", "step-2").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_unknown_to_step() {
    let mut host = SequenceHost::default();
    assert!(matches!(host.connect_steps("step-1", "nope").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_mismatched_slot_scope() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    assert!(matches!(host.connect_steps("step-2", "step-4").unwrap_err(), SequenceCoreError::MismatchedSlotScope));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_cycle() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.connect_steps("step-2", "step-3").expect("connect step-2 to step-3");
    assert!(matches!(host.connect_steps("step-3", "step-1").unwrap_err(), SequenceCoreError::CycleDetected));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rewires_existing_incoming_edge() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.connect_steps("step-3", "step-2").expect("rewire onto step-2");
    assert_eq!(host.snapshot.edges.len(), 1);
    assert_eq!(host.snapshot.edges[0].from, "step-3");
    assert_eq!(host.snapshot.edges[0].to, "step-2");
}

#[semio_framework_async_macros::async_test]
async fn disconnect_steps_returns_false_when_no_matching_edge() {
    let mut host = SequenceHost::default();
    assert!(!host.disconnect_steps("step-2", "step-1"));
    assert_eq!(host.snapshot.edges.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn load_json_parses_valid_fixture() {
    let json = SequenceHost::default().to_json().expect("fixture json");
    let host = SequenceHost::load_json(&json).expect("load json");
    assert_eq!(host.snapshot.steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn load_json_rejects_unsupported_schema() {
    let result = SequenceHost::load_json(r#"{"schema":"other","steps":[],"edges":[]}"#);
    assert!(matches!(result, Err(SequenceCoreError::UnsupportedSchema(schema)) if schema == "other"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_json_reports_imperative_catalogue_schema() {
    let host = SequenceHost::default();
    assert!(host.catalogue_json().contains("\"imperative.catalogue\""));
}

#[semio_framework_async_macros::async_test]
async fn layout_expanded_slots_positions_slot_members_relative_to_owner() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    host.layout_expanded_slots();
    let child = host.snapshot.steps.iter().find(|step| step.id == "step-4").unwrap();
    assert_eq!(child.x, 400.0);
    assert_eq!(child.y, 160.0);
}

#[semio_framework_async_macros::async_test]
async fn reorganize_syncs_step_positions_from_dag_layout() {
    let mut host = SequenceHost::default();
    host.reorganize(&DagLayoutOptions::default()).expect("reorganize");
    for step in &host.snapshot.steps {
        let node = host.dag.fixture.nodes.iter().find(|node| node.id == step.id).expect("node for step");
        assert_eq!(step.x, node.x);
        assert_eq!(step.y, node.y);
    }
}

#[semio_framework_async_macros::async_test]
async fn pick_step_id_at_screen_finds_step_under_cursor() {
    let host = SequenceHost::default();
    let id = host.pick_step_id_at_screen(400.0, 300.0, 800, 600, 1.0);
    assert_eq!(id, Some("step-1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn pick_step_id_at_screen_returns_none_when_missing_all_nodes() {
    let host = SequenceHost::default();
    let id = host.pick_step_id_at_screen(-9000.0, -9000.0, 800, 600, 1.0);
    assert_eq!(id, None);
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_when_owner_collapsed() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: true });
    let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_for_non_control_owner() {
    let mut host = SequenceHost::default();
    let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("step-2"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_for_unknown_owner_id() {
    let mut host = SequenceHost::default();
    let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("nope"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn build_path_returns_unordered_slot_body_when_multiple_heads() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-5".into(), kind: "log.print".into(), params: StepParams::new(), x: 280.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    let path = host.build_path();
    let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
    let body = control.bodies.get("then").expect("then body");
    assert_eq!(body.steps.len(), 2);
    assert!(body.steps.iter().any(|step| step.id == "step-4"));
    assert!(body.steps.iter().any(|step| step.id == "step-5"));
}

#[semio_framework_async_macros::async_test]
async fn step_to_dag_node_shows_collapsed_indicator_for_collapsed_control_step() {
    let mut host = SequenceHost::default();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    let expanded = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
    assert_eq!(expanded.abbreviation, "▾️0");
    host.set_step_collapsed("step-3", true);
    let collapsed = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
    assert_eq!(collapsed.abbreviation, "▸️0");
}

#[semio_framework_async_macros::async_test]
async fn set_ghost_step_and_clear_ghost_step_toggle_dag_ghost_node() {
    let mut host = SequenceHost::default();
    assert!(host.dag.ghost_node().is_none());
    host.set_ghost_step("math.add", 10.0, 20.0);
    assert!(host.dag.ghost_node().is_some());
    host.clear_ghost_step();
    assert!(host.dag.ghost_node().is_none());
}

#[semio_framework_async_macros::async_test]
async fn run_executes_default_snapshot_and_records_scope() {
    let host = SequenceHost::default();
    let result = host.run();
    assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
    assert!(!result.effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn compile_text_renders_default_snapshot_steps() {
    let host = SequenceHost::default();
    let text = host.compile_text();
    assert!(text.contains("state.set"));
    assert!(text.contains("log.print"));
}

#[semio_framework_async_macros::async_test]
async fn compiled_wire_literal_includes_step_ids() {
    let host = SequenceHost::default();
    let literal = host.compiled_wire_literal();
    assert!(literal.contains("step-1"));
    assert!(literal.contains("step-2"));
}

#[semio_framework_async_macros::async_test]
async fn sequence_io_declares_the_steps_in_port() {
    let io = sequence_io();
    assert_eq!(io.document_schema, SEQUENCE_DOCUMENT_SCHEMA);
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "steps:in");
    assert_eq!(port.direction, semio_framework::MediaPortDirection::In);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert!(!port.required);
}

#[semio_framework_async_macros::async_test]
async fn next_available_step_id_is_free_and_deterministic() {
    let fixture = default_snapshot();
    let id = next_available_step_id(&fixture);
    assert!(!fixture.to_fixture().steps.iter().any(|step| step.id == id));
    assert_eq!(id, next_available_step_id(&fixture), "pure function of the fixture, not a mutating counter");
}
//#endregion 🔖️HostTests
