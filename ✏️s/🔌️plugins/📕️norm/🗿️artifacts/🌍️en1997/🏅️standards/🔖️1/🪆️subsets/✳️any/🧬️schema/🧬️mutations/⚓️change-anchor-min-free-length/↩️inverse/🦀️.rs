//! ↩️ `change-anchor-min-free-length` inverse: restores the previous `min_free_length`.

use crate::change_anchor_min_free_length::ChangeAnchorMinFreeLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorMinFreeLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_min_free_length::ChangeAnchorMinFreeLength> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorMinFreeLength { id: payload.id.clone(), new_min_free_length: row.min_free_length })],
        None => Vec::new(),
    })
}
