//! 🚚️ Generation2d command — `move-media-node`: one widget dropped at a canvas position (palette, agent), committed through
//! the ONE node-drag machine as ONE relative `move-nodes` leaf — the offset from the widget's BASE position — so editing
//! the move in history replays it on whatever base it lands on (design §13.3).

use crate::editor::generation2d::commands::node_graph_edit::{generation2d_node_drag_emit, generation2d_node_drag_leaves};
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::NodeDragRecord;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ The verb a `moveMediaNode` tool transaction is scoped by.
pub const MOVE_MEDIA_NODE_VERB: &str = "moveMediaNode";

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// ✋️ Drops `node_id` at `(x, y)`: a widget the graph lacks or one without a stored position is refused by name, a
/// non-finite drop is refused, a drop where it already sits leaves zero trace.
pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    if !host_snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == payload.node_id) {
        return Err(Fault::from(format!("moveMediaNode refusal: the graph has no widget {:?}", payload.node_id)));
    }
    let base = host_snapshot.layout.get(&payload.node_id).ok_or_else(|| Fault::from(format!("moveMediaNode refusal: widget {:?} has no stored position", payload.node_id)))?;
    let (dx, dy) = (payload.x - base.x, payload.y - base.y);
    if !dx.is_finite() || !dy.is_finite() {
        return Err(Fault::from("moveMediaNode refusal: the dropped position must yield a finite offset".to_owned()));
    }
    let leaves = generation2d_node_drag_leaves(host_snapshot, &[NodeDragRecord { gesture_id: MOVE_MEDIA_NODE_VERB.into(), node_ids: vec![payload.node_id.clone()], dx, dy }]);
    Ok(generation2d_node_drag_emit(doc, MOVE_MEDIA_NODE_VERB, MOVE_MEDIA_NODE_VERB, leaves))
}
