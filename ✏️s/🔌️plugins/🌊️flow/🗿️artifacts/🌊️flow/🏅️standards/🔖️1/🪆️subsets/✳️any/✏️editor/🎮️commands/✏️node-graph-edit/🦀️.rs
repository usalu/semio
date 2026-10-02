//! 🕸️ 🎯️ Flow play app commands command — `node-graph-edit`.

use crate::editor::flow::modes::edit::tools::drag::flow_drag_tool_emit;
use crate::editor::flow::commands::patch_flow_widgets::widget_field_leaf;
use semio_framework_tool_machine::{node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow, NodePortSide};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::{apply_canvas_options, flow_content_leaves, flow_content_leaves_emit, seed_host_catalogue, sync_host_selection_domains, FLOW_GRAPH_OPERATION_RAW_BYTES};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::{neural::ColdRetire, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️FlowNodeGraphEditOp
/// 🎯️ One batched edit inside a `FlowCommand::NodeGraphEdit`/`SpotlightCommit`, closed over the exact
/// operation rows published by the shared NodeGraph renderer (design §13.3): every row names its entities by id, and no
/// row carries a whole fixture.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum)]
pub enum FlowNodeGraphEditOp {
    #[dsl(key = "delete")]
    Delete { node_ids: Vec<String>, synapse_ids: Vec<String> },
    #[dsl(key = "insert-port")]
    InsertPort { node_id: String, side: String, index: u32 },
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect")]
    Disconnect { synapse_id: String },
    #[dsl(key = "move")]
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    #[dsl(key = "set-slider")]
    SetSlider { widget_id: String, value: f64 },
}

impl From<NodeGraphEditRow> for FlowNodeGraphEditOp {
    fn from(row: NodeGraphEditRow) -> Self {
        match row {
            NodeGraphEditRow::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => Self::Connect { source_node_id, source_port_id, target_node_id, target_port_id },
            NodeGraphEditRow::Disconnect { synapse_id } => Self::Disconnect { synapse_id },
            NodeGraphEditRow::Move(NodeDragRecord { gesture_id, node_ids, dx, dy }) => Self::Move { gesture_id, node_ids, dx, dy },
            NodeGraphEditRow::SetSlider { widget_id, value } => Self::SetSlider { widget_id, value },
            NodeGraphEditRow::InsertPort { node_id, side, index } => Self::InsertPort { node_id, side: if side == NodePortSide::Input { "input" } else { "output" }.into(), index },
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => Self::Delete { node_ids, synapse_ids },
        }
    }
}

/// 🧾 Decodes the renderer's rows through the ONE node-graph edit decoder every guest shares
/// (`semio_framework_tool_machine::node_graph_edit_rows`, design §13.3), within the retained route's wire authority. Every
/// row must decode or the entire batch is refused before a retained operation is admitted.
pub fn operations_from_action(args: &dsl::DslValue) -> Result<Vec<FlowNodeGraphEditOp>, Fault> {
    if dsl::json::to_json_string(args).len() > FLOW_GRAPH_OPERATION_RAW_BYTES {
        return Err(Fault::from(format!("nodeGraphEdit arguments exceed the {FLOW_GRAPH_OPERATION_RAW_BYTES}-byte wire authority")));
    }
    let rows = node_graph_edit_rows(args).map_err(|reason| Fault::from(format!("nodeGraphEdit refusal: {reason}")))?;
    Ok(rows.into_iter().map(FlowNodeGraphEditOp::from).collect())
}
//#endregion 🔖️FlowNodeGraphEditOp

//#region 🔖️SharedDispatch
/// 🪪️ The verb a `nodeGraphEdit` release's drag-tool transaction is scoped by.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// ✋️ The node-graph gesture records a batch's `move` rows are — the release of a host-previewed drag, in row order.
pub fn node_graph_edit_drags(operations: &[FlowNodeGraphEditOp]) -> Vec<NodeDragRecord> {
    operations
        .iter()
        .filter_map(|operation| match operation {
            FlowNodeGraphEditOp::Move { gesture_id, node_ids, dx, dy } => Some(NodeDragRecord { gesture_id: gesture_id.clone(), node_ids: node_ids.clone(), dx: *dx, dy: *dy }),
            _ => None,
        })
        .collect()
}

/// 🎚️ The ABSOLUTE `set-node-param` leaves of a batch's `setSlider` rows on `content` (design §13.1: for a slider the intent
/// is the value), in row order: one per row naming a slider whose value it changes. A dragged knob carries its press as the
/// dispatch's own `gesture`/`commit`, so the framework scrub machine keeps every tick provisional and commits the release as
/// ONE child edit (design §12); this never reads a gesture.
pub fn node_graph_edit_slider_leaves(content: &SemioFlowSnapshot, operations: &[FlowNodeGraphEditOp]) -> Vec<SemioFlowMutation> {
    operations
        .iter()
        .filter_map(|operation| match operation {
            FlowNodeGraphEditOp::SetSlider { widget_id, value } => content.nodes.iter().find(|node| node.id == *widget_id).and_then(|node| widget_field_leaf(node, "value", &value.to_string())),
            _ => None,
        })
        .collect()
}

/// 🕸️ The child edit of one `nodeGraphEdit` batch (design §12, §13.3). `move` rows never touch the working host: they are
/// a drag's release (the node-graph gesture record), committed through the node-drag machine as relative `drag-nodes`
/// leaves, in ONE tool transaction after whatever the rest of the batch lands (a wire the same gesture drew). `setSlider`
/// rows never touch it either: they land as absolute `set-node-param` leaves ([`node_graph_edit_slider_leaves`]). Every
/// other row edits the working host by the ids it names, and the change lands as the intent leaves that turn the content
/// child into the edited scene ([`flow_content_leaves`]) — never a whole-content `set-snapshot`.
pub fn node_graph_edit_result(doc: &ArtifactView<'_, FlowSnapshot>, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp]) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let child_id = &doc.snapshot.content.child_id;
    let content = doc.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    let (widgets, synapses, layout) = crate::working_from_flow_content_snapshot(&content);
    let live = semio_framework_artifact_flow_flow::FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: config.camera.clone(), widgets, synapses, layout };
    let mut host = flow::flow_host_with_session(&live, session);
    live.retire_cold();
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    let drags = node_graph_edit_drags(operations);
    let sliders = node_graph_edit_slider_leaves(&content, operations);
    let edited = (|| {
        let mut edited = false;
        for sub_operation in operations {
            match sub_operation {
                FlowNodeGraphEditOp::Delete { node_ids, synapse_ids } => {
                    sync_host_selection_domains(&mut host, node_ids, synapse_ids, &[]);
                    host.delete_selection().map_err(|error| Fault::from(format!("nodeGraphEdit delete refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::InsertPort { node_id, side, index } => {
                    let index = *index as usize;
                    match side.as_str() {
                        "input" => host.add_input_port(node_id, index),
                        _ => host.add_output_port(node_id, index),
                    }
                    .map_err(|error| Fault::from(format!("nodeGraphEdit insertPort refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).map_err(|error| Fault::from(format!("nodeGraphEdit connect refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Disconnect { synapse_id } => {
                    host.disconnect(synapse_id).map_err(|error| Fault::from(format!("nodeGraphEdit disconnect refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Move { .. } | FlowNodeGraphEditOp::SetSlider { .. } => continue,
            }
            edited = true;
        }
        Ok::<bool, Fault>(edited)
    })();
    let edits = match edited {
        Ok(true) => flow_content_leaves(&content, &crate::flow_content_snapshot_from_working(&host.host_snapshot.widgets, &host.host_snapshot.synapses, &host.host_snapshot.layout)),
        Ok(false) => Vec::new(),
        Err(error) => {
            host.retire_cold();
            return Err(error);
        }
    };
    host.retire_cold();
    let leaves: Vec<SemioFlowMutation> = edits.into_iter().chain(sliders).collect();
    if !drags.is_empty() {
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        return Ok(flow_drag_tool_emit(child_id, NODE_GRAPH_EDIT_VERB, authoring_seed, &content, leaves, &drags));
    }
    Ok(flow_content_leaves_emit(child_id, &leaves))
}
//#endregion 🔖️SharedDispatch

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

/// 🕸️ The command body: the batch's child edit ([`node_graph_edit_result`]) against the main window's config.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    node_graph_edit_result(doc, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
