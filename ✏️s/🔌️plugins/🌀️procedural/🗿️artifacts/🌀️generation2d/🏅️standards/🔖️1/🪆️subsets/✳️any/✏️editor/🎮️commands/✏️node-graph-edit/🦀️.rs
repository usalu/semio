//! 🕸️ Generation2d command — `node-graph-edit`: the one verb both node-graph hosts dispatch for graph edits.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::host_operations;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_slider_value, move_nodes};
use semio_framework_tool_machine::{node_drag_commit, NodeDragRecord, NODE_DRAG_OPERATION};
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

/// 🧾️ The sub-operations the node-graph surfaces dispatch, and the leaf each one yields:
/// - `setSlider {widgetId, value}` — the inline slider overlay's value: the ABSOLUTE `change-slider-value` leaf. A dragged
///   knob carries its press as the dispatch's own top-level `gesture`/`commit`, so the framework scrub machine (design
///   §13.1) keeps every tick provisional and commits the release as ONE edit; this handler never reads a gesture.
/// - `move {gestureId, nodeIds, dx, dy}` — the node-graph gesture record of a released node drag (design §13.3): the
///   relative `move-nodes` leaf, committed through the ONE node-drag machine as ONE tool transaction.
/// - `setHostSnapshot`, `connect`, `disconnect`, `deleteSelection` — one-shot structural edits, authored as the host diff of
///   the graph they leave behind. When operator kinds are not yet contributed, the host rebuild can drop unresolved wires
///   before a `disconnect` runs; the canvas still names the document synapse, so that cut falls back to a document-level
///   `disconnect-synapse`.
fn apply_operations(doc: &ArtifactView<'_, Generation2dSnapshot>, sub_operations: &[dsl::json::Value], selected: &[String]) -> Emit<Generation2dMutation, Generation2dConfigMutation> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    let operation = |row: &dsl::json::Value| row.get("operation").and_then(|value| value.as_str()).unwrap_or("").to_string();
    let structural: Vec<&dsl::json::Value> = sub_operations.iter().filter(|row| matches!(operation(row).as_str(), "setHostSnapshot" | "deleteSelection" | "connect" | "disconnect")).collect();
    let mut document_disconnects = Vec::new();
    let mut leaves = if structural.is_empty() {
        Vec::new()
    } else {
        host_operations(host_snapshot, |host| {
            for row in &structural {
                match operation(row).as_str() {
                    "setHostSnapshot" => {
                        if let Some(host_snapshot) = row.get("hostSnapshotJson").and_then(|value| value.as_str()).and_then(|json| semio_framework_os_flow::os_pack::json::from_json_str::<FlowHostSnapshot>(json).ok()) {
                            host.replace_host_snapshot(host_snapshot);
                        }
                    }
                    "deleteSelection" => {
                        for id in selected {
                            let _ = host.remove_widget(id);
                        }
                    }
                    "connect" => {
                        let text = |key: &str| row.get(key).and_then(|value| value.as_str());
                        if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (text("sourceNodeId"), text("sourcePortId"), text("targetNodeId"), text("targetPortId")) {
                            let _ = host.connect_ports(from, from_port, to, to_port);
                        }
                    }
                    _ => {
                        if let Some(synapse_id) = row.get("synapseId").and_then(|value| value.as_str()) {
                            if host.disconnect(synapse_id).is_err() && host_snapshot.synapses.iter().any(|synapse| synapse.id == synapse_id) {
                                document_disconnects.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse(synapse_id.to_string()));
                            }
                        }
                    }
                }
            }
        })
    };
    leaves.extend(document_disconnects);
    leaves.extend(sub_operations.iter().filter(|row| operation(row) == "setSlider").filter_map(|row| slider_leaf(host_snapshot, row)));
    let records: Vec<NodeDragRecord> = sub_operations.iter().filter(|row| operation(row) == NODE_DRAG_OPERATION).filter_map(|row| NodeDragRecord::from_row(&dsl::json::to_dsl_value(row)).ok()).collect();
    let sliders_only = !sub_operations.is_empty() && sub_operations.iter().all(|row| operation(row) == "setSlider");
    let ui_scope = if sliders_only { slider_gesture_ui_scope() } else { UiDirtyScope::default() };
    if records.is_empty() {
        return Emit { artifact_mutations: leaves, ui_scope, ..Default::default() };
    }
    leaves.extend(generation2d_node_drag_leaves(host_snapshot, &records));
    let gesture = records.first().map_or(NODE_GRAPH_EDIT_VERB, |record| record.gesture_id.as_str());
    generation2d_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, gesture, leaves)
}

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 🛠️ ONE tool transaction of `leaves` through the ONE node-drag machine of `🛠️tool-machine` (design §13.3): the ref
/// minted from the admission's authoring seed, the host clock and `<appId>#<verb>`, for the press `gesture`. A view
/// without command authority publishes the leaves plainly; nothing yielded is the empty emit (zero trace).
pub(crate) fn generation2d_node_drag_emit(doc: &ArtifactView<'_, Generation2dSnapshot>, verb: &str, gesture: &str, leaves: Vec<Generation2dMutation>) -> Emit<Generation2dMutation, Generation2dConfigMutation> {
    let authoring_seed = doc.operation().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
    match node_drag_commit(format!("{}#{verb}", crate::editor::generation2d::GENERATION2D_EDITOR_APP_ID), protocol::ActorId(authoring_seed.clone()), gesture, leaves, clock) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, leaves),
        Some((_, leaves)) => Emit::mutations(leaves),
        None => Emit::default(),
    }
}

/// 🎚️ The ABSOLUTE `change-slider-value` leaf of one `setSlider` row, or nothing for a malformed row, a widget that is no
/// slider, or the value the slider already holds.
pub(crate) fn slider_leaf(host_snapshot: &FlowHostSnapshot, row: &dsl::json::Value) -> Option<Generation2dMutation> {
    let widget_id = row.get("widgetId").and_then(|value| value.as_str())?;
    let value = row.get("value").and_then(dsl::json::Value::as_f64).filter(|value| value.is_finite())?;
    host_snapshot
        .widgets
        .iter()
        .any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::InputSlider { id, value: current, .. } if id == widget_id && *current != value))
        .then(|| change_slider_value(widget_id, value))
}

/// 🚚️ The `move-nodes` leaves `records` mean on `host_snapshot`, one per record that moves, each over the record's nodes the
/// graph holds — a record naming no widget, or one whose offset moves nothing, yields no leaf.
pub(crate) fn generation2d_node_drag_leaves(host_snapshot: &FlowHostSnapshot, records: &[NodeDragRecord]) -> Vec<Generation2dMutation> {
    records
        .iter()
        .filter(|record| record.moves())
        .filter_map(|record| {
            let ids: Vec<String> = record.node_ids.iter().filter(|id| host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == id.as_str())).cloned().collect();
            (!ids.is_empty()).then(|| move_nodes(ids, record.dx, record.dy))
        })
        .collect()
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
    Ok(apply_operations(doc, &sub_operations, selected))
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
