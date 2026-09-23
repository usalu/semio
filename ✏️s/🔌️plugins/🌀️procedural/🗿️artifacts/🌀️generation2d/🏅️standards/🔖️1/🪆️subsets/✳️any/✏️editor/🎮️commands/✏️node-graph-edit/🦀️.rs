//! 🕸️ 🕸️ Generation2d play app commands command — `node-graph-edit`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::host_operations;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🌉️ `operations_json` is a locally-defined array of sub-operation descriptors (not a framework
/// boundary type) — parsed generically via `pack::json`'s raw tree, not `serde_json`.
fn parse_sub_operations(text: &str) -> Vec<dsl::json::Value> {
    dsl::json::parse(text).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default()
}

/// 🧾️ The sub-operations the node-graph surfaces dispatch: `setHostSnapshot`, `move` / `connect` /
/// `disconnect`, `deleteSelection`, and `setSlider`. `move`, `disconnect`, and `setSlider` used to
/// fall through `_ => {}`, so canvas drags, wire cuts and slider ticks were silent no-ops
/// (ticket 26/09/23/FLOW-AND-PROCEDURAL-FEATURE-COMPLETE gaps #1/#8).
fn apply_operations(host_snapshot: &FlowHostSnapshot, sub_operations: &[dsl::json::Value], selected: &[String]) -> Emit<Generation2dMutation, Generation2dConfigMutation> {
    let mut document_disconnects = Vec::new();
    let operations = host_operations(host_snapshot, |host| {
        for operation in sub_operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setHostSnapshot" => {
                    if let Some(host_snapshot) = operation.get("hostSnapshotJson").and_then(|value| value.as_str()).and_then(|json| semio_framework_os_flow::os_pack::json::from_json_str::<FlowHostSnapshot>(json).ok()) {
                        host.replace_host_snapshot(host_snapshot);
                    }
                }
                "deleteSelection" => {
                    for id in selected {
                        let _ = host.remove_widget(id);
                    }
                }
                "connect" => {
                    let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                    let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                    let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                    let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                        let _ = host.connect_ports(from, from_port, to, to_port);
                    }
                }
                "disconnect" => {
                    if let Some(synapse_id) = operation.get("synapseId").and_then(|value| value.as_str()) {
                        // ✂️ When operator kinds are not yet contributed, `FlowHost` rebuild can drop
                        // unresolved wires before this arm runs; the canvas still names the document
                        // synapse id, so fall back to a document-level disconnect mutation.
                        if host.disconnect(synapse_id).is_err() && host_snapshot.synapses.iter().any(|synapse| synapse.id == synapse_id) {
                            document_disconnects.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse(synapse_id.to_string()));
                        }
                    }
                }
                "move" => {
                    let node_id = operation.get("nodeId").and_then(|value| value.as_str());
                    let x = operation.get("x").and_then(dsl::json::Value::as_f64);
                    let y = operation.get("y").and_then(dsl::json::Value::as_f64);
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        let _ = host.move_widget(node_id, x, y);
                    }
                }
                "setSlider" => {
                    let widget_id = operation.get("widgetId").and_then(|value| value.as_str());
                    let value = operation.get("value").and_then(dsl::json::Value::as_f64);
                    if let (Some(widget_id), Some(value)) = (widget_id, value) {
                        host.set_slider_value(widget_id, value);
                    }
                }
                _ => {}
            }
        }
    });
    let mut operations = operations;
    operations.extend(document_disconnects);
    let coalesce_key = gesture_coalesce_key(sub_operations);
    let ui_scope = if coalesce_key.is_some() { slider_gesture_ui_scope() } else { UiDirtyScope::default() };
    Emit { artifact_mutations: operations, coalesce_key, ui_scope, ..Default::default() }
}

/// 🐢️ What ONE slider tick invalidates: the graph that draws the knob, the preview that re-evaluates,
/// and the two panels that read the moved value back.
pub(crate) fn slider_gesture_ui_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![
            crate::editor::generation2d::modes::edit::windows::flow::GENERATION2D_PLAY_BODY_MAIN.to_string(),
            crate::editor::generation2d::modes::edit::windows::preview::GENERATION2D_PLAY_BODY_PREVIEW.to_string(),
        ],
        panel_bodies: vec![
            crate::editor::generation2d::panels::inspection::GENERATION2D_PLAY_BODY_INSPECTION.to_string(),
            crate::editor::generation2d::panels::document::GENERATION2D_PLAY_BODY_ARTIFACT.to_string(),
        ],
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🎚️ The coalesce key one continuous gesture's edits fold under, or `None` for a discrete edit.
fn gesture_coalesce_key(sub_operations: &[dsl::json::Value]) -> Option<String> {
    let mut gesture: Option<&str> = None;
    for operation in sub_operations {
        if operation.get("operation").and_then(|value| value.as_str()) != Some("setSlider") {
            return None;
        }
        let key = operation.get("gesture").and_then(|value| value.as_str())?;
        if gesture.is_some_and(|current| current != key) {
            return None;
        }
        gesture = Some(key);
    }
    gesture.map(|key| format!("graph-slider:{key}"))
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation2dPlayApp::handle` always routes this
/// command through `apply` below instead), so `"deleteSelection"` sub-operations degrade to treating
/// the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}

pub fn apply_selected(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation2dSnapshot>, selected: &[String]) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.host_snapshot, &sub_operations, selected))
}

/// 🕹️ `"deleteSelection"` reads the `graph` domain's current selection instead of a deleted config
/// field — no config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// `graph`'s selection.
pub fn apply(
    payload: &NodeGraphEdit,
    doc: &ArtifactView<'_, Generation2dSnapshot>,
    _cfg: &ConfigView<'_, Generation2dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    apply_selected(payload, doc, &interaction.selection("graph").ids)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
