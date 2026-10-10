//! 🪟️ 🧩️ Flow play app commands command — `move-media-node`: one node dropped at a canvas position, committed through the
//! edit-mode drag tool as ONE relative `drag-nodes` leaf on the composed content child.

use crate::editor::flow::modes::edit::tools::drag::flow_drag_tool_emit;
use semio_framework_tool_machine::NodeDragRecord;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// 🪪️ The verb a `moveMediaNode` drag-tool transaction is scoped by.
pub const MOVE_MEDIA_NODE_VERB: &str = "moveMediaNode";

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// ✋️ Drops `node_id` at `(x, y)`: the offset from its BASE position is the leaf, so editing the move in history replays
/// it on whatever base it lands on. A node the flow lacks is refused by name; a drop where it already sits is no edit.
pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let child_id = &doc.snapshot.content.child_id;
    let content = doc.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    let node = content.nodes.iter().find(|node| node.id == payload.node_id).ok_or_else(|| Fault::from(format!("moveMediaNode refusal: the flow has no node {:?}", payload.node_id)))?;
    let (dx, dy) = (payload.x - node.position.x, payload.y - node.position.y);
    if !dx.is_finite() || !dy.is_finite() { return Err(Fault::from("moveMediaNode refusal: the dropped position must yield a finite offset".to_owned())); }
    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    Ok(flow_drag_tool_emit(child_id, MOVE_MEDIA_NODE_VERB, authoring_seed, &content, Vec::new(), &[NodeDragRecord { gesture_id: MOVE_MEDIA_NODE_VERB.into(), node_ids: vec![payload.node_id.clone()], dx, dy }]))
}
