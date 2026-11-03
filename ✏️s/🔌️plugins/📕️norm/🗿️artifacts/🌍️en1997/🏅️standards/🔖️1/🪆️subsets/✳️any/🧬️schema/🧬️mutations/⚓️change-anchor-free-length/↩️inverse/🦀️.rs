//! ↩️ `change-anchor-free-length` inverse: restores the previous `free_length`.

use crate::change_anchor_free_length::ChangeAnchorFreeLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFreeLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_free_length::ChangeAnchorFreeLength> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorFreeLength { id: payload.id.clone(), new_free_length: row.free_length })],
        None => Vec::new(),
    })
}
