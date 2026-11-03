//! ↩️ `change-frame-node-support` inverse: restores the previous `support`.

use crate::change_frame_node_support::ChangeFrameNodeSupport;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameNodeSupport, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_node_support::ChangeFrameNodeSupport> {
    Ok(match base.frame_nodes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameNodeSupport { id: payload.id.clone(), new_support: row.support })],
        None => Vec::new(),
    })
}
