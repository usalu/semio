//! ↩️ `change-frame-bar-end-node-id` inverse: restores the previous `end_node_id`.

use crate::change_frame_bar_end_node_id::ChangeFrameBarEndNodeId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameBarEndNodeId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_bar_end_node_id::ChangeFrameBarEndNodeId> {
    Ok(match base.frame_bars.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameBarEndNodeId { id: payload.id.clone(), new_end_node_id: row.end_node_id.clone() })],
        None => Vec::new(),
    })
}
