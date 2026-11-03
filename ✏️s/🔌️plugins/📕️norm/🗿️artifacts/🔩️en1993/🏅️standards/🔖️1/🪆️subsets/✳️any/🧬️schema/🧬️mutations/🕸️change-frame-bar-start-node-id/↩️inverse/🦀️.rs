//! ↩️ `change-frame-bar-start-node-id` inverse: restores the previous `start_node_id`.

use crate::change_frame_bar_start_node_id::ChangeFrameBarStartNodeId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameBarStartNodeId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_bar_start_node_id::ChangeFrameBarStartNodeId> {
    Ok(match base.frame_bars.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameBarStartNodeId { id: payload.id.clone(), new_start_node_id: row.start_node_id.clone() })],
        None => Vec::new(),
    })
}
