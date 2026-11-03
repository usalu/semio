//! ↩️ `change-anchor-head-depth` inverse: restores the previous `head_depth`.

use crate::change_anchor_head_depth::ChangeAnchorHeadDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorHeadDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_head_depth::ChangeAnchorHeadDepth> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorHeadDepth { id: payload.id.clone(), new_head_depth: row.head_depth })],
        None => Vec::new(),
    })
}
