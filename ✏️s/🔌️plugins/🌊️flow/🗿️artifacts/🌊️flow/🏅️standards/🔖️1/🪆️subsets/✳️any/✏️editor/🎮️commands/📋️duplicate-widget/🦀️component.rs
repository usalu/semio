//! 📋️ Flow widget duplication as a replayable, bounded child-content continuation.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, RequestId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
use serde_json::json;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
pub const DUPLICATE_WIDGET_STEP_ACTION_ID: &str = "duplicateWidgetStep";
pub(crate) const MAX_COLLISION_ROWS_PER_STEP: usize = 64;
const MAX_CHECKPOINT_BYTES: usize = 4_096;
const MAX_WIDGET_ID_BYTES: usize = 256;
const MAX_NODE_PARAMS: usize = 32;
const MAX_NODE_ENCODED_BYTES: usize = 3_072;
//#endregion 🔖️Constants

//#region 🔖️Payloads
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "duplicate-widget")]
pub struct DuplicateWidget {
    pub widget_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-widget-step")]
pub struct DuplicateWidgetStep {
    pub app_id: String,
    pub document_id: String,
    pub operation_id: String,
    pub child_id: String,
    pub generation: u64,
    pub phase: String,
    pub scan_index: u64,
    pub suffix: u64,
    pub candidate_id: String,
    pub source_index: Option<u64>,
    pub new_id: Option<String>,
    pub base_revision: String,
    pub child_revision: String,
}
//#endregion 🔖️Payloads

//#region 🔖️Revision
fn revision_id(revision: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(64);
    for byte in revision {
        result.push(HEX[usize::from(byte >> 4)] as char);
        result.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    result
}
//#endregion 🔖️Revision

//#region 🔖️Search
fn widget_candidate(source_id: &str, suffix: u64) -> String {
    if suffix <= 1 {
        format!("{source_id}-copy")
    } else {
        format!("{source_id}-copy-{suffix}")
    }
}

fn synapse_candidate(source_id: &str, new_id: &str, suffix: u64) -> String {
    if suffix <= 1 {
        format!("{source_id}-to-{new_id}")
    } else {
        format!("{source_id}-to-{new_id}-{suffix}")
    }
}

fn bounded_node(node: &FlowNode) -> bool {
    if node.id.len() > MAX_WIDGET_ID_BYTES || node.kind.len() > MAX_WIDGET_ID_BYTES || node.label.len() > MAX_WIDGET_ID_BYTES || node.params.len() > MAX_NODE_PARAMS {
        return false;
    }
    let mut bytes = node.id.len().saturating_add(node.kind.len()).saturating_add(node.label.len());
    for param in &node.params {
        if param.key.len() > MAX_WIDGET_ID_BYTES || param.value.len() > MAX_WIDGET_ID_BYTES {
            return false;
        }
        bytes = bytes.saturating_add(param.key.len()).saturating_add(param.value.len());
    }
    bytes <= MAX_NODE_ENCODED_BYTES
}

enum SearchOutcome {
    Yield(DuplicateWidgetStep),
    Commit { node: FlowNode, source_id: String, new_id: String, synapse_id: String },
    Busy,
    Cancel,
}

fn advance_search(mut step: DuplicateWidgetStep, scene: &SemioFlowSnapshot) -> SearchOutcome {
    let start = usize::try_from(step.scan_index).unwrap_or(usize::MAX);
    match step.phase.as_str() {
        "source" => {
            let start = start.min(scene.nodes.len());
            let end = start.saturating_add(MAX_COLLISION_ROWS_PER_STEP).min(scene.nodes.len());
            if let Some((offset, _)) = scene.nodes[start..end].iter().enumerate().find(|(_, node)| node.id == step.candidate_id) {
                step.source_index = Some((start + offset) as u64);
                step.phase = "widget".into();
                step.scan_index = 0;
                step.suffix = 1;
            } else if end == scene.nodes.len() {
                return SearchOutcome::Cancel;
            } else {
                step.scan_index = end as u64;
            }
        }
        "widget" => {
            let candidate = widget_candidate(&step.candidate_id, step.suffix);
            let start = start.min(scene.nodes.len());
            let end = start.saturating_add(MAX_COLLISION_ROWS_PER_STEP).min(scene.nodes.len());
            if scene.nodes[start..end].iter().any(|node| node.id == candidate) {
                let Some(suffix) = step.suffix.checked_add(1) else { return SearchOutcome::Cancel };
                step.suffix = suffix;
                step.scan_index = 0;
            } else if end == scene.nodes.len() {
                step.new_id = Some(candidate);
                step.phase = "synapse".into();
                step.scan_index = 0;
                step.suffix = 1;
            } else {
                step.scan_index = end as u64;
            }
        }
        "synapse" => {
            let Some(new_id) = step.new_id.clone() else { return SearchOutcome::Cancel };
            let candidate = synapse_candidate(&step.candidate_id, &new_id, step.suffix);
            let start = start.min(scene.edges.len());
            let end = start.saturating_add(MAX_COLLISION_ROWS_PER_STEP).min(scene.edges.len());
            if scene.edges[start..end].iter().any(|edge| edge.id == candidate) {
                let Some(suffix) = step.suffix.checked_add(1) else { return SearchOutcome::Cancel };
                step.suffix = suffix;
                step.scan_index = 0;
            } else if end == scene.edges.len() {
                let Some(source) = step.source_index.and_then(|index| usize::try_from(index).ok()).and_then(|index| scene.nodes.get(index)) else { return SearchOutcome::Cancel };
                if source.id != step.candidate_id {
                    return SearchOutcome::Cancel;
                }
                if !bounded_node(source) {
                    return SearchOutcome::Busy;
                }
                let mut node = source.clone();
                node.id = new_id.clone();
                return SearchOutcome::Commit { node, source_id: step.candidate_id, new_id, synapse_id: candidate };
            } else {
                step.scan_index = end as u64;
            }
        }
        _ => return SearchOutcome::Cancel,
    }
    SearchOutcome::Yield(step)
}
//#endregion 🔖️Search

//#region 🔖️Continuation
fn request_id(payload: &DuplicateWidgetStep) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64 ^ payload.generation;
    for byte in payload.operation_id.bytes().chain(payload.phase.bytes()).chain(payload.candidate_id.bytes()) {
        digest = (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3);
    }
    digest ^ payload.scan_index.rotate_left(17) ^ payload.suffix.rotate_left(31)
}

fn queue(payload: &DuplicateWidgetStep) -> Effect {
    Effect::DispatchAction { req: RequestId(request_id(payload)), action: DUPLICATE_WIDGET_STEP_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(json!(payload))), delay_ms: 0 }
}

fn yield_step(step: DuplicateWidgetStep) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let json = serde_json::to_string(&step).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.checkpoint-invalid"), error.to_string()))?;
    if json.len() > MAX_CHECKPOINT_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.checkpoint-too-large"), "the bounded Flow continuation checkpoint exceeds 4,096 UTF-8 bytes"));
    }
    Ok(Emit { config_mutations: vec![FlowConfigMutation::SetDuplicateWidgetProgress { json }], effects: vec![queue(&step)], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

fn checkpoint_generation(json: &str) -> Option<u64> {
    if json.len() > MAX_CHECKPOINT_BYTES {
        return None;
    }
    serde_json::from_str::<DuplicateWidgetStep>(json).ok().map(|step| step.generation)
}

fn commit_duplicate(generation: u64, child_id: &str, node: FlowNode, source_id: String, new_id: String, synapse_id: String) -> Emit<FlowMutation, FlowConfigMutation> {
    let edge = FlowEdge { id: synapse_id, from: PortRef { node: source_id, port: String::new() }, to: PortRef { node: new_id, port: String::new() }, kind: "data".into() };
    Emit {
        child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, vec![SemioFlowMutation::InsertNode(insert_node::InsertNode { node }), SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge })])],
        coalesce_key: Some(format!("duplicateWidget:{generation}")),
        config_mutations: vec![FlowConfigMutation::SetDuplicateWidgetProgress { json: String::new() }],
        ui_scope: UiDirtyScope::Full,
        ..Default::default()
    }
}

pub fn advance_duplicate_widget(payload: &DuplicateWidgetStep, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let operation = doc.operation()?;
    if cfg.snapshot.duplicate_widget_progress_json.len() > MAX_CHECKPOINT_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.checkpoint-invalid"), "the stored Flow continuation exceeds 4,096 UTF-8 bytes"));
    }
    if payload.app_id.len() > 10
        || payload.document_id.len() > MAX_WIDGET_ID_BYTES
        || payload.operation_id.len() > 32
        || payload.child_id.len() > MAX_WIDGET_ID_BYTES
        || payload.phase.len() > 16
        || payload.candidate_id.len() > MAX_WIDGET_ID_BYTES
        || payload.new_id.as_ref().is_some_and(|id| id.len() > MAX_WIDGET_ID_BYTES)
        || payload.base_revision.len() != 64
        || payload.child_revision.len() != 64
    {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.checkpoint-invalid"), "the Flow continuation exceeds its bounded field envelope"));
    }
    if serde_json::from_str::<DuplicateWidgetStep>(&cfg.snapshot.duplicate_widget_progress_json).ok().as_ref() != Some(payload)
        || payload.app_id != operation.app_instance_id.to_string()
        || payload.document_id != operation.parent_document_id
        || payload.operation_id.parse::<u64>().is_err()
        || payload.child_id != doc.snapshot.content.child_id
        || payload.base_revision != operation.canonical_base_revision_hex()
    {
        return Ok(Emit::default());
    }
    if revision_id(doc.children.revision("content", &payload.child_id)?) != payload.child_revision {
        return Ok(Emit { config_mutations: vec![FlowConfigMutation::CancelDuplicateWidget { generation: payload.generation }], ..Default::default() });
    }
    let scene = doc.children.typed_read::<SemioFlowSnapshot>("content", &payload.child_id)?;
    match advance_search(payload.clone(), &scene) {
        SearchOutcome::Yield(next) => yield_step(next),
        SearchOutcome::Commit { node, source_id, new_id, synapse_id } => Ok(commit_duplicate(payload.generation, &payload.child_id, node, source_id, new_id, synapse_id)),
        SearchOutcome::Busy => Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.busy"), "the source widget exceeds the bounded duplicate envelope")),
        SearchOutcome::Cancel => Ok(Emit { config_mutations: vec![FlowConfigMutation::SetDuplicateWidgetProgress { json: String::new() }], ..Default::default() }),
    }
}
//#endregion 🔖️Continuation

//#region 🔖️Handlers
pub fn handle(payload: &DuplicateWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _eval: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    if payload.widget_id.is_empty() || payload.widget_id.len() > MAX_WIDGET_ID_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.busy"), "the Flow widget id must fit the 256-byte admission envelope"));
    }
    doc.children.typed_read::<SemioFlowSnapshot>("content", &doc.snapshot.content.child_id)?;
    let operation = doc.operation()?;
    let child_revision = revision_id(doc.children.revision("content", &doc.snapshot.content.child_id)?);
    let step = DuplicateWidgetStep {
        app_id: operation.app_instance_id.to_string(),
        document_id: operation.parent_document_id.clone(),
        operation_id: operation.operation_id.to_string(),
        child_id: doc.snapshot.content.child_id.clone(),
        generation: operation.generation,
        phase: "source".into(),
        candidate_id: payload.widget_id.clone(),
        base_revision: operation.canonical_base_revision_hex(),
        child_revision,
        ..Default::default()
    };
    let mut emit = yield_step(step)?;
    if let Some(generation) = checkpoint_generation(&cfg.snapshot.duplicate_widget_progress_json) {
        emit.config_mutations.insert(0, FlowConfigMutation::CancelDuplicateWidget { generation });
    }
    Ok(emit)
}

pub fn handle_step(payload: &DuplicateWidgetStep, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _eval: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    advance_duplicate_widget(payload, doc, cfg)
}
//#endregion 🔖️Handlers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app_with_registry, register_content_child, FlowApp};
    use crate::editor::flow::FlowCommand;
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

    fn next_checkpoint(result: InvocationResult) -> Option<serde_json::Value> {
        result.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == DUPLICATE_WIDGET_STEP_ACTION_ID => args.map(store::pack_rt::dsl_value_to_json),
            _ => None,
        })
    }

    async fn source_id(app: &FlowApp) -> String {
        let fixture = app.snapshot().await.expect("Flow snapshot").to_fixture();
        crate::artifacts::flow::schema::widget_id(fixture.widgets.first().expect("default Flow widget")).to_string()
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
        let encoded = serde_json::to_string(&step).expect("checkpoint encode");
        let decoded: DuplicateWidgetStep = serde_json::from_str(&encoded).expect("checkpoint decode");
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
        let first = initial.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": widget_id })), &meta("document-a")).await.expect("public Flow duplicate start");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow public start handler/op-codec/diff/apply envelope exceeded 8 ms");
        let checkpoint = next_checkpoint(first).expect("durable Flow continuation");
        let (_, config_ops, _) = initial.take_last_emit_wire().await.expect("initial Flow config operation wire");

        let mut restarted = flow_app_with_registry().await;
        let child_id = restarted.snapshot().await.expect("restarted Flow snapshot").content.child_id;
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
        second_app.handle_action("addWidget", Some(&serde_json::json!({ "kind": "inputNote", "x": 7.0, "y": 7.0 })), &meta("document-b")).await.expect("make second Flow document distinct");
        register_content_child(&mut second_app).await;
        let first_id = source_id(&first_app).await;
        let second_id = source_id(&second_app).await;
        assert_ne!(first_app.snapshot().await.expect("first Flow document").content.child_id, second_app.snapshot().await.expect("second Flow document").content.child_id);
        let first = first_app.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": first_id })), &meta("document-a")).await.expect("first Flow document start");
        let first_checkpoint = next_checkpoint(first).expect("first Flow checkpoint");
        let second = second_app.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": second_id })), &meta("document-b")).await.expect("second Flow document start");
        let second_checkpoint = next_checkpoint(second).expect("second Flow checkpoint");

        let first_generation = serde_json::from_value::<DuplicateWidgetStep>(first_checkpoint.clone()).expect("first checkpoint decode").generation;
        let replacement_id = source_id(&first_app).await;
        let started = std::time::Instant::now();
        let replacement = first_app.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": replacement_id })), &meta("document-a")).await.expect("superseding Flow start");
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
        let first_snapshot = first.snapshot().await.expect("first Flow parent");
        let second_snapshot = second.snapshot().await.expect("second Flow parent");
        assert_eq!(first_snapshot.content.child_id, second_snapshot.content.child_id, "fixture deliberately shares one child identity across two parents");
        let widget_id = source_id(&first).await;
        let first_checkpoint = next_checkpoint(first.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": widget_id })), &meta("parent-a")).await.expect("first parent start")).expect("first parent checkpoint");
        let second_id = source_id(&second).await;
        let second_checkpoint = next_checkpoint(second.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": second_id })), &meta("parent-b")).await.expect("second parent start")).expect("second parent checkpoint");
        let first_payload: DuplicateWidgetStep = serde_json::from_value(first_checkpoint.clone()).expect("first parent payload");
        let second_payload: DuplicateWidgetStep = serde_json::from_value(second_checkpoint.clone()).expect("second parent payload");
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
        let first = app.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": widget_id })), &meta("document-stale")).await.expect("Flow stale test start");
        let checkpoint = next_checkpoint(first).expect("Flow stale checkpoint");
        app.handle_action("addWidget", Some(&serde_json::json!({ "kind": "inputNote", "x": 1.0, "y": 1.0 })), &meta("document-stale")).await.expect("advance Flow content identity");
        register_content_child(&mut app).await;
        let started = std::time::Instant::now();
        let stale = app.handle_action(DUPLICATE_WIDGET_STEP_ACTION_ID, Some(&checkpoint), &meta("document-stale")).await.expect("stale Flow continuation no-op");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow stale content/ABA rejection exceeded 8 ms");
        assert!(stale.requested_effects.is_empty() && stale.mutations.is_empty());

        let started = std::time::Instant::now();
        let oversized = app.handle_action("duplicateWidget", Some(&serde_json::json!({ "widgetId": "x".repeat(MAX_WIDGET_ID_BYTES + 1) })), &meta("document-stale")).await;
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Flow oversize admission exceeded 8 ms");
        let fault = oversized.expect_err("oversize Flow admission must be explicit Busy");
        assert_eq!(fault.code.0, "flow.duplicate-widget.busy");
    }
}
//#endregion 🧪️Tests
