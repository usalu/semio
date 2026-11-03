//! ↩️ `change-frame-node-load-x` inverse: restores the previous `load_x`.

use crate::change_frame_node_load_x::ChangeFrameNodeLoadX;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeLoadX, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_load_x::ChangeFrameNodeLoadX> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeLoadX { id: payload.id.clone(), new_load_x: row.load_x })],
        None => Vec::new(),
    })
}
