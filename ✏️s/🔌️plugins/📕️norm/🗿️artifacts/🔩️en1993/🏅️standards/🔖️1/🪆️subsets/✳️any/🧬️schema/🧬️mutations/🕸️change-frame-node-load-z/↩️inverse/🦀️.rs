//! ↩️ `change-frame-node-load-z` inverse: restores the previous `load_z`.

use crate::change_frame_node_load_z::ChangeFrameNodeLoadZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeLoadZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_load_z::ChangeFrameNodeLoadZ> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeLoadZ { id: payload.id.clone(), new_load_z: row.load_z })],
        None => Vec::new(),
    })
}
