//! ↩️ `change-frame-node-x` inverse: restores the previous `x`.

use crate::change_frame_node_x::ChangeFrameNodeX;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeX, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_x::ChangeFrameNodeX> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeX { id: payload.id.clone(), new_x: row.x })],
        None => Vec::new(),
    })
}
