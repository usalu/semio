//! 📋️ Flow widget duplication as a replayable, bounded child-content continuation.

use crate::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, RequestId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
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
    Effect::DispatchAction { req: RequestId(request_id(payload)), action: DUPLICATE_WIDGET_STEP_ACTION_ID.into(), args: Some(dsl::ToValue::to_value(payload)), delay_ms: 0 }
}

fn yield_step(step: DuplicateWidgetStep) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let json = dsl::os_pack::json::to_json_string(&step);
    if json.len() > MAX_CHECKPOINT_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("flow.duplicate-widget.checkpoint-too-large"), "the bounded Flow continuation checkpoint exceeds 4,096 UTF-8 bytes"));
    }
    Ok(Emit { config_mutations: vec![FlowConfigMutation::SetDuplicateWidgetProgress { json }], effects: vec![queue(&step)], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

fn checkpoint_generation(json: &str) -> Option<u64> {
    if json.len() > MAX_CHECKPOINT_BYTES {
        return None;
    }
    dsl::os_pack::json::from_json_str::<DuplicateWidgetStep>(json).ok().map(|step| step.generation)
}

fn commit_duplicate(generation: u64, child_id: &str, node: FlowNode, source_id: String, new_id: String, synapse_id: String) -> Emit<FlowMutation, FlowConfigMutation> {
    let edge = FlowEdge { id: synapse_id, from: PortRef { node: source_id, port: String::new() }, to: PortRef { node: new_id, port: String::new() }, kind: "data".into() };
    Emit {
        child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, &[SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)), SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(edge))])],
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
    if dsl::os_pack::json::from_json_str::<DuplicateWidgetStep>(&cfg.snapshot.duplicate_widget_progress_json).ok().as_ref() != Some(payload)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
