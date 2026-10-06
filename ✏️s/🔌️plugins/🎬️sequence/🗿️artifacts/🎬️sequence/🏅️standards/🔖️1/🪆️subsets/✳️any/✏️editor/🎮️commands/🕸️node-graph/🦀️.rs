//! 🕸️ Sequence play app commands — bulk node-graph edits and viewport pan/zoom.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::sequence::{sequence_child_leaves_emit, sequence_child_leaves_from_host_mutation, SEQUENCE_PLAY_APP_ID};
use crate::mutations::SequenceMutation;
use crate::{SequenceCamera, SequenceSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_pack_json::{self as json, Value};
use semio_framework_tool_machine::{node_drag_emit, NodeDragRecord, NodeGraphEditRow};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    /// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
    pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

    /// 🧾️ Decodes one host row through the ONE shared node-graph row decoder of `🛠️tool-machine`; the `setSlider` and
    /// `insertPort` rows a sequence has no widget for are refused by name, so the whole batch is refused.
    pub(crate) fn sequence_node_graph_row(row: &semio_framework_value::DslValue) -> Result<NodeGraphEditRow, Fault> {
        match NodeGraphEditRow::from_row(row).map_err(|reason| crate::editor::sequence::sequence_fault("sequence.node-graph.malformed", format!("sequence nodeGraphEdit refusal: {reason}")))? {
            NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => Err(crate::editor::sequence::sequence_fault("sequence.node-graph.unsupported", "sequence nodeGraphEdit refusal: a sequence has no sliders and no variadic ports")),
            row => Ok(row),
        }
    }

    /// 🧾️ Every row edits the editor host by the ids it names, and the published edit is the child INTENT leaves that carry
    /// the content from where it was to where the host left it (`sequence_content_leaves`), never a whole-content snapshot:
    /// - `move` — the node-graph gesture record (design §13.3): its steps move by the ONE offset, landing as relative
    ///   `drag-nodes`; a drag commits through the ONE node-drag machine as ONE composed-child tool transaction (design §12);
    /// - `connect`, `disconnect`, `delete` — one-shot structural edits of the steps and edges they name.
    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let rows: Vec<NodeGraphEditRow> = match json::parse(&payload.operations_json, json::JsonMemberPolicy::Reject) {
            Ok(Value::Array(rows)) => rows.iter().map(|row| sequence_node_graph_row(&json::to_dsl_value(row))).collect::<Result<_, _>>()?,
            _ => return Err(crate::editor::sequence::sequence_fault("sequence.node-graph.malformed", "sequence nodeGraphEdit operations must be a JSON array")),
        };
        let records: Vec<NodeDragRecord> = rows.iter().filter_map(|row| if let NodeGraphEditRow::Move(record) = row { Some(record.clone()) } else { None }).filter(NodeDragRecord::moves).collect();
        let leaves = sequence_child_leaves_from_host_mutation(doc, |host| {
            if !records.is_empty() {
                let mut next = host.snapshot.clone();
                for record in &records {
                    for step in next.steps.iter_mut().filter(|step| record.node_ids.contains(&step.id)) {
                        step.x += record.dx;
                        step.y += record.dy;
                    }
                }
                let _ = host.replace_snapshot(next);
            }
            for row in &rows {
                match row {
                    NodeGraphEditRow::Connect { source_node_id, target_node_id, .. } => {
                        let _ = host.connect_steps(source_node_id, target_node_id);
                    }
                    NodeGraphEditRow::Disconnect { synapse_id } => {
                        if let Some(edge) = host.snapshot.edges.iter().find(|edge| &edge.id == synapse_id).cloned() {
                            host.disconnect_steps(&edge.from, &edge.to);
                        }
                    }
                    NodeGraphEditRow::Delete { node_ids, synapse_ids } => {
                        for edge in host.snapshot.edges.iter().filter(|edge| synapse_ids.contains(&edge.id)).cloned().collect::<Vec<_>>() {
                            host.disconnect_steps(&edge.from, &edge.to);
                        }
                        for step_id in node_ids {
                            host.remove_step(step_id);
                        }
                    }
                    NodeGraphEditRow::Move(_) | NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => {}
                }
            }
        })?;
        let dragged = leaves.iter().any(|leaf| matches!(leaf, SemioFlowMutation::DragNodes(_)));
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        if !dragged || authoring_seed.is_empty() {
            return Ok(sequence_child_leaves_emit(doc.snapshot, leaves));
        }
        let Some(gesture) = records.first().map(|record| record.gesture_id.as_str()) else { return Ok(sequence_child_leaves_emit(doc.snapshot, leaves)) };
        let drag = node_drag_emit(SEQUENCE_PLAY_APP_ID, NODE_GRAPH_EDIT_VERB, authoring_seed, gesture, leaves);
        Ok(Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::node_drag_child::<SemioFlowSnapshot, _>(drag, "content", &doc.snapshot.content.child_id) })
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️SetViewport
pub mod set_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "set-viewport")]
    pub struct SetViewport {
        #[dsl(block)]
        pub camera: SequenceCamera,
    }

    pub fn handle(_payload: &SetViewport, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetViewport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️history/🦀️.rs"]
mod history_tests;
//#endregion 🧪️Tests
