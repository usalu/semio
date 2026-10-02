//! 🕸️ Generation2d command — `node-graph-edit`: the one verb both node-graph hosts dispatch for graph edits.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::host_operations;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_slider_value, move_nodes};
use semio_framework_tool_machine::{node_drag_commit, node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow, NodePortSide};
use crate::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🧾️ The rows of one `nodeGraphEdit` batch through the ONE shared closed decoder (`🛠️tool-machine`
/// [`node_graph_edit_rows`], design §13.3): a whole fixture (`setHostSnapshot`), an ambient-selection delete, an absolute
/// move, an unknown operation or any malformed row refuses the whole batch before anything is authored.
pub fn rows(payload: &NodeGraphEdit) -> Result<Vec<NodeGraphEditRow>, Fault> {
    let operations = dsl::json::parse(&payload.operations_json).map_err(|error| Fault::from(format!("nodeGraphEdit operations are not JSON: {error}")))?;
    node_graph_edit_rows(&dsl::DslValue::object([("operations".to_string(), dsl::json::to_dsl_value(&operations))])).map_err(Fault::from)
}

/// ✂️ Cuts the wire `synapse_id`. When operator kinds are not yet contributed, the host rebuild can drop unresolved wires
/// before the cut runs; the canvas still names the document synapse, so that cut lands as a document-level
/// `disconnect-synapse`. A wire neither holds is refused.
fn cut(host: &mut FlowHost, host_snapshot: &FlowHostSnapshot, synapse_id: &str, document_cuts: &mut Vec<Generation2dMutation>) -> Result<(), String> {
    match host.disconnect(synapse_id) {
        Ok(()) => Ok(()),
        Err(_) if host_snapshot.synapses.iter().any(|synapse| synapse.id == synapse_id) => {
            document_cuts.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse(synapse_id.to_string()));
            Ok(())
        }
        Err(error) => Err(error.to_string()),
    }
}

/// 🧾️ The leaves one decoded batch yields:
/// - `setSlider {widgetId, value}` — the inline slider overlay's value: the ABSOLUTE `change-slider-value` leaf. A dragged
///   knob carries its press as the dispatch's own top-level `gesture`/`commit`, so the framework scrub machine (design
///   §13.1) keeps every tick provisional and commits the release as ONE edit; this handler never reads a gesture.
/// - `move {gestureId, nodeIds, dx, dy}` — the node-graph gesture record of a released node drag (design §13.3): the
///   relative `move-nodes` leaf, committed through the ONE node-drag machine as ONE tool transaction.
/// - `connect`, `disconnect`, `insertPort`, `delete {nodeIds, synapseIds}` — structural edits by the ids they name, authored
///   as the id-keyed leaves of the graph they leave behind; a refused host edit refuses the batch.
fn apply_rows(doc: &ArtifactView<'_, Generation2dSnapshot>, rows: &[NodeGraphEditRow]) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    let structural = rows.iter().any(|row| matches!(row, NodeGraphEditRow::Connect { .. } | NodeGraphEditRow::Disconnect { .. } | NodeGraphEditRow::InsertPort { .. } | NodeGraphEditRow::Delete { .. }));
    let mut document_cuts = Vec::new();
    let mut refusal = None;
    let mut leaves = if structural {
        host_operations(host_snapshot, |host| {
            refusal = rows
                .iter()
                .try_for_each(|row| match row {
                    NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).map(drop).map_err(|error| error.to_string()),
                    NodeGraphEditRow::Disconnect { synapse_id } => cut(host, host_snapshot, synapse_id, &mut document_cuts),
                    NodeGraphEditRow::InsertPort { node_id, side: NodePortSide::Input, index } => host.add_input_port(node_id, *index as usize).map_err(|error| error.to_string()),
                    NodeGraphEditRow::InsertPort { node_id, side: NodePortSide::Output, index } => host.add_output_port(node_id, *index as usize).map_err(|error| error.to_string()),
                    NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                        synapse_ids.iter().try_for_each(|synapse_id| cut(host, host_snapshot, synapse_id, &mut document_cuts))?;
                        node_ids.iter().try_for_each(|node_id| host.remove_widget(node_id).map_err(|error| error.to_string()))
                    }
                    NodeGraphEditRow::Move(_) | NodeGraphEditRow::SetSlider { .. } => Ok(()),
                })
                .err();
        })
    } else {
        Vec::new()
    };
    if let Some(reason) = refusal {
        for leaf in leaves.into_iter().chain(document_cuts) {
            leaf.retire_cold();
        }
        return Err(Fault::from(format!("nodeGraphEdit refusal: {reason}")));
    }
    leaves.extend(document_cuts);
    leaves.extend(rows.iter().filter_map(|row| match row {
        NodeGraphEditRow::SetSlider { widget_id, value } => slider_leaf(host_snapshot, widget_id, *value),
        _ => None,
    }));
    let records: Vec<NodeDragRecord> = rows.iter().filter_map(|row| match row {
        NodeGraphEditRow::Move(record) => Some(record.clone()),
        _ => None,
    }).collect();
    let sliders_only = !rows.is_empty() && rows.iter().all(|row| matches!(row, NodeGraphEditRow::SetSlider { .. }));
    let ui_scope = if sliders_only { slider_gesture_ui_scope() } else { UiDirtyScope::default() };
    if records.is_empty() {
        return Ok(Emit { artifact_mutations: leaves, ui_scope, ..Default::default() });
    }
    leaves.extend(generation2d_node_drag_leaves(host_snapshot, &records));
    let gesture = records.first().map_or(NODE_GRAPH_EDIT_VERB, |record| record.gesture_id.as_str());
    Ok(generation2d_node_drag_emit(doc, NODE_GRAPH_EDIT_VERB, gesture, leaves))
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

/// 🎚️ The ABSOLUTE `change-slider-value` leaf of one `setSlider` row, or nothing for a widget that is no slider or the
/// value the slider already holds.
pub(crate) fn slider_leaf(host_snapshot: &FlowHostSnapshot, widget_id: &str, value: f64) -> Option<Generation2dMutation> {
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

/// 🕹️ The one entry every route takes — `app_commands!`'s generated dispatch and the retained reducer alike: the decoded
/// rows name every entity they edit, so no route reads an ambient selection.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    apply_rows(doc, &rows(payload)?)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
