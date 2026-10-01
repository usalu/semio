//! 🪟️ 🧩️ Flow play app commands command — `patch-flow-widgets`: a widget value (a slider's `value`, a note's `text`) set as
//! the ABSOLUTE `set-node-param` leaf of the composed content child (design §13.1: for a value the intent is the value). A
//! dragged control carries its press as the dispatch's own `gesture`/`commit`, so the framework scrub machine keeps every tick
//! provisional and commits the release as ONE child edit stamped with its `TransactionRef` (design §12); this never reads a
//! gesture and never coalesces.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{app::ChildEmit, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{set_node_param::SetNodeParam, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowNode, SemioFlowSnapshot};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🎚️ The ABSOLUTE `set-node-param` leaf that sets `field` of `node` to the raw UI input `raw`: a slider's `value` (parsed,
/// finite, written in the node's own number form), a note's `text` verbatim. `None` for a field the widget does not carry,
/// an unparsable number, or the value the widget already holds — a control back at its start yields nothing.
pub fn widget_field_leaf(node: &FlowNode, field: &str, raw: &str) -> Option<SemioFlowMutation> {
    let current = node.params.iter().find(|param| param.key == field).map(|param| param.value.as_str());
    let value = match (field, node.kind.as_str()) {
        ("value", "inputSlider") => raw.parse::<f64>().ok().filter(|value| value.is_finite() && current.and_then(|current| current.parse::<f64>().ok()) != Some(*value))?.to_string(),
        ("text", "inputNote") => (current != Some(raw)).then(|| raw.to_string())?,
        _ => return None,
    };
    Some(SemioFlowMutation::SetNodeParam(SetNodeParam { id: node.id.clone(), key: field.into(), value }))
}

/// 🧮️ The leaves `payload` means on `content`: one per addressed node, in content order, whose field it changes.
pub fn patch_flow_widgets_leaves(content: &SemioFlowSnapshot, payload: &PatchFlowWidgets) -> Vec<SemioFlowMutation> {
    content.nodes.iter().filter(|node| payload.widget_ids.contains(&node.id)).filter_map(|node| widget_field_leaf(node, &payload.field, &payload.value)).collect()
}

/// 📮️ ONE plain edit of `leaves` on the content child `child_id`; nothing changed is the empty emit (zero trace).
pub fn widget_leaves_emit(child_id: &str, leaves: &[SemioFlowMutation]) -> Emit<FlowMutation, NoConfigMutation> {
    if leaves.is_empty() {
        return Emit::default();
    }
    Emit { child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, leaves)], ui_scope: UiDirtyScope::Full, ..Default::default() }
}

pub fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let child_id = &doc.snapshot.content.child_id;
    let content = doc.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    Ok(widget_leaves_emit(child_id, &patch_flow_widgets_leaves(&content, payload)))
}
