//! ↩️ `change-frame-node-z` inverse: restores the previous `z`.

use crate::change_frame_node_z::ChangeFrameNodeZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_z::ChangeFrameNodeZ> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeZ { id: payload.id.clone(), new_z: row.z })],
        None => Vec::new(),
    })
}
