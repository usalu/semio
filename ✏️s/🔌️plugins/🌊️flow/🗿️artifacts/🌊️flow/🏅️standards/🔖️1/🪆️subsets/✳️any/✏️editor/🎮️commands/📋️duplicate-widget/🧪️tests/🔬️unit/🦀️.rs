
use super::*;
use crate::editor::flow::FlowCommand;
use crate::editor::flow::testkit::{FlowApp, flow_app_with_registry, register_content_child};
use semio_framework_plugin::testkit::meta;
use semio_framework_plugin::{InvocationResult, PluginApp};
use store::SpaceMember;

fn node(id: &str) -> FlowNode {
    FlowNode { id: id.into(), kind: "inputNote".into(), label: "Note".into(), params: Vec::new(), position: Default::default() }
}

fn search_step(phase: &str) -> DuplicateWidgetStep {
    DuplicateWidgetStep {
        app_id: "1".into(),
        document_id: "doc".into(),
        operation_id: "duplicateWidget".into(),
        child_id: "child".into(),
        generation: 1,
        phase: phase.into(),
        candidate_id: "note".into(),
        base_revision: "0".repeat(64),
        child_revision: "1".repeat(64),
        ..Default::default()
    }
}

fn next_checkpoint(result: InvocationResult) -> Option<dsl::DslValue> {
    result.requested_effects.into_iter().find_map(|effect| match effect {
        Effect::DispatchAction { action, args, .. } if action == DUPLICATE_WIDGET_STEP_ACTION_ID => args,
        _ => None,
    })
}

async fn source_id(app: &FlowApp) -> String {
    let fixture = app.snapshot().expect("Flow snapshot").to_fixture();
    crate::schema::widget_id(fixture.widgets.first().expect("default Flow widget")).to_string()
}

async fn reidentify_parent(app: &mut FlowApp, parent_id: &str) {
    let files = app.document_pack().await.expect("Flow parent pack");
    let mut parsed: store::ParsedDocumentText<FlowSnapshot, FlowMutation> = store::parse_document_pack(&files.pack, &files.spr).await.expect("parse Flow parent pack");
    parsed.envelope.id = parent_id.into();
    let changed = store::print_document_pack(&parsed.envelope).await.expect("print reidentified Flow parent");
    app.load_document_pack(&changed).await.expect("load reidentified Flow parent");
}

#[test]
fn dense_collision_search_consumes_only_sixty_four_rows() {
    let scene = SemioFlowSnapshot { nodes: (0..10_000).map(|index| node(&format!("node-{index}"))).collect(), ..Default::default() };
    let step = search_step("source");
    let SearchOutcome::Yield(next) = advance_search(step, &scene) else { panic!("bounded continuation") };
    assert_eq!(next.scan_index, MAX_COLLISION_ROWS_PER_STEP as u64);
    assert_eq!(next.phase, "source");
}

#[test]
fn revision_and_terminal_node_work_have_hard_size_credits() {
    assert_eq!(revision_id([7; 32]).len(), 64);
    assert!(bounded_node(&node("note")));
    let mut oversized = node("note");
    oversized.label = "x".repeat(MAX_NODE_ENCODED_BYTES + 1);
    assert!(!bounded_node(&oversized));
}

#[test]
fn checkpoint_contains_complete_restart_state_without_a_process_map() {
    let mut step = search_step("synapse");
    step.source_index = Some(7);
    step.new_id = Some("note-copy".into());
    step.scan_index = 64;
    let encoded = dsl::os_pack::json::to_json_string(&step);
    let decoded: DuplicateWidgetStep = dsl::os_pack::json::from_json_str(&encoded).expect("checkpoint decode");
    assert_eq!(decoded, step);
    assert!(encoded.len() <= MAX_CHECKPOINT_BYTES);
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_replays_checkpoint_into_a_fresh_composed_app_under_eight_ms() {
    let mut initial = flow_app_with_registry().await;
    let widget_id = source_id(&initial).await;
    let command = FlowCommand::DuplicateWidget(DuplicateWidget { widget_id: widget_id.clone() });
    let started = std::time::Instant::now();
    let command_wire = <FlowCommand as protocol::OpBinary>::encode_op(&command).expect("Flow command encode");
    assert_eq!(<FlowCommand as protocol::OpBinary>::decode_op(&command_wire).expect("Flow command decode"), command);
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Flow public command codec envelope exceeded 8 ms");

    let started = std::time::Instant::now();
    let first = initial.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": widget_id }))), &meta("document-a")).await.expect("public Flow duplicate start");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow public start handler/op-codec/diff/apply envelope exceeded 8 ms");
    let checkpoint = next_checkpoint(first).expect("durable Flow continuation");
    let (_, config_ops, _) = initial.take_last_emit_wire().await.expect("initial Flow config operation wire");

    let mut restarted = flow_app_with_registry().await;
    let child_id = restarted.snapshot().expect("restarted Flow snapshot").content.child_id;
    let before = restarted.child_store("content", &child_id).await.expect("restarted Flow child").document_pack_bytes().await.expect("Flow child before pack");
    let started = std::time::Instant::now();
    let config_blobs = protocol::decode_ops_vec(&config_ops).expect("Flow config operation vector decode");
    for blob in &config_blobs {
        <FlowConfigMutation as protocol::OpBinary>::decode_op(blob).expect("Flow config operation decode");
    }
    restarted.resume_task_emit(Vec::new(), config_ops, Vec::new(), &meta("document-a")).await.expect("replay Flow config operation");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow operation decode/diff/apply replay exceeded 8 ms");

    let mut next = Some(checkpoint);
    for _ in 0..128 {
        let Some(args) = next.take() else { break };
        let started = std::time::Instant::now();
        let result = restarted.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&args), &meta("document-a")).await.expect("public Flow continuation");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow continuation handler/op-codec/diff/apply envelope exceeded 8 ms");
        next = next_checkpoint(result);
    }
    assert!(next.is_none(), "Flow duplicate must finish within the bounded continuation budget");
    let after = restarted.child_store("content", &child_id).await.expect("restarted Flow child").document_pack_bytes().await.expect("Flow child after pack");
    assert_ne!(after, before, "the replayed public action must reach the real child diff/apply path");
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_isolates_two_documents_and_emits_generation_bound_supersession() {
    let mut first_app = flow_app_with_registry().await;
    let mut second_app = flow_app_with_registry().await;
    second_app.handle_action("addWidget", Some(&dsl::DslValue::from(serde_json::json!({ "kind": "inputNote", "x": 7.0, "y": 7.0 }))), &meta("document-b")).await.expect("make second Flow document distinct");
    register_content_child(&mut second_app).await;
    let first_id = source_id(&first_app).await;
    let second_id = source_id(&second_app).await;
    assert_ne!(first_app.snapshot().expect("first Flow document").content.child_id, second_app.snapshot().expect("second Flow document").content.child_id);
    let first = first_app.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": first_id }))), &meta("document-a")).await.expect("first Flow document start");
    let first_checkpoint = next_checkpoint(first).expect("first Flow checkpoint");
    let second = second_app.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": second_id }))), &meta("document-b")).await.expect("second Flow document start");
    let second_checkpoint = next_checkpoint(second).expect("second Flow checkpoint");

    let first_generation = <DuplicateWidgetStep as dsl::FromValue>::from_value(first_checkpoint.clone()).expect("first checkpoint decode").generation;
    let replacement_id = source_id(&first_app).await;
    let started = std::time::Instant::now();
    let replacement = first_app.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": replacement_id }))), &meta("document-a")).await.expect("superseding Flow start");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow supersession envelope exceeded 8 ms");
    assert!(next_checkpoint(replacement).is_some());
    let (_, config_ops, _) = first_app.take_last_emit_wire().await.expect("supersession config operation wire");
    let decoded = protocol::decode_ops_vec(&config_ops).expect("supersession operation vector").iter().map(|blob| <FlowConfigMutation as protocol::OpBinary>::decode_op(blob).expect("supersession operation decode")).collect::<Vec<_>>();
    assert!(decoded.iter().any(|mutation| matches!(mutation, FlowConfigMutation::CancelDuplicateWidget { generation } if *generation == first_generation)), "supersession must publish an exact generation-bound cancellation outcome");

    let stale = first_app.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&first_checkpoint), &meta("document-a")).await.expect("superseded continuation no-op");
    assert!(stale.requested_effects.is_empty() && stale.mutations.is_empty());
    let other = second_app.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&second_checkpoint), &meta("document-b")).await.expect("independent Flow continuation");
    assert!(next_checkpoint(other).is_some(), "one document's supersession must not cancel another app/document");
}

#[semio_framework_async_macros::async_test]
async fn shared_child_identity_under_two_parents_cannot_cross_resume_or_cancel() {
    let mut first = flow_app_with_registry().await;
    let mut second = flow_app_with_registry().await;
    reidentify_parent(&mut first, "flow-parent-a").await;
    reidentify_parent(&mut second, "flow-parent-b").await;
    let first_snapshot = first.snapshot().expect("first Flow parent");
    let second_snapshot = second.snapshot().expect("second Flow parent");
    assert_eq!(first_snapshot.content.child_id, second_snapshot.content.child_id, "fixture deliberately shares one child identity across two parents");
    let widget_id = source_id(&first).await;
    let first_checkpoint = next_checkpoint(first.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": widget_id }))), &meta("parent-a")).await.expect("first parent start")).expect("first parent checkpoint");
    let second_id = source_id(&second).await;
    let second_checkpoint = next_checkpoint(second.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": second_id }))), &meta("parent-b")).await.expect("second parent start")).expect("second parent checkpoint");
    let first_payload: DuplicateWidgetStep = dsl::FromValue::from_value(first_checkpoint.clone()).expect("first parent payload");
    let second_payload: DuplicateWidgetStep = dsl::FromValue::from_value(second_checkpoint.clone()).expect("second parent payload");
    assert_ne!(first_payload.document_id, second_payload.document_id);
    assert_eq!(first_payload.child_id, second_payload.child_id);
    let crossed = second.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&first_checkpoint), &meta("parent-b")).await.expect("cross-parent continuation is rejected");
    assert!(crossed.requested_effects.is_empty() && crossed.mutations.is_empty());
    let own = second.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&second_checkpoint), &meta("parent-b")).await.expect("own parent continuation survives");
    assert!(next_checkpoint(own).is_some());
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_rejects_stale_content_and_oversize_admission() {
    let mut app = flow_app_with_registry().await;
    let widget_id = source_id(&app).await;
    let first = app.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": widget_id }))), &meta("document-stale")).await.expect("Flow stale test start");
    let checkpoint = next_checkpoint(first).expect("Flow stale checkpoint");
    app.handle_action("addWidget", Some(&dsl::DslValue::from(serde_json::json!({ "kind": "inputNote", "x": 1.0, "y": 1.0 }))), &meta("document-stale")).await.expect("advance Flow content identity");
    register_content_child(&mut app).await;
    let started = std::time::Instant::now();
    let stale = app.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&checkpoint), &meta("document-stale")).await.expect("stale Flow continuation no-op");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow stale content/ABA rejection exceeded 8 ms");
    assert!(stale.requested_effects.is_empty() && stale.mutations.is_empty());

    let started = std::time::Instant::now();
    let oversized = app.handle_action("duplicateWidget", Some(&dsl::DslValue::from(serde_json::json!({ "widgetId": "x".repeat(MAX_WIDGET_ID_BYTES + 1) }))), &meta("document-stale")).await;
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow oversize admission exceeded 8 ms");
    let fault = oversized.expect_err("oversize Flow admission must be explicit Busy");
    assert_eq!(fault.code.0, "flow.duplicate-widget.busy");
}
