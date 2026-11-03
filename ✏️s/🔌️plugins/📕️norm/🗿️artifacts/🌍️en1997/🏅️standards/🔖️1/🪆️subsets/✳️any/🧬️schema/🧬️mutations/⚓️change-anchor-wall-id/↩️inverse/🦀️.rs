//! ↩️ `change-anchor-wall-id` inverse: restores the previous `wall_id`.

use crate::change_anchor_wall_id::ChangeAnchorWallId;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorWallId, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_wall_id::ChangeAnchorWallId> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorWallId { id: payload.id.clone(), new_wall_id: row.wall_id.clone() })],
        None => Vec::new(),
    })
}
