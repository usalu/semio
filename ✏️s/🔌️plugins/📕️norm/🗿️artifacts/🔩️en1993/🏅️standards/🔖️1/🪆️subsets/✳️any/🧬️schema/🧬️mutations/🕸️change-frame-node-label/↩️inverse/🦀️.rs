//! ↩️ `change-frame-node-label` inverse: restores the previous `label`.

use crate::change_frame_node_label::ChangeFrameNodeLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_label::ChangeFrameNodeLabel> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
