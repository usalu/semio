//! ↩️ `change-anchor-fixed-length` inverse: restores the previous `fixed_length`.

use crate::change_anchor_fixed_length::ChangeAnchorFixedLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorFixedLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_fixed_length::ChangeAnchorFixedLength> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorFixedLength { id: payload.id.clone(), new_fixed_length: row.fixed_length })],
        None => Vec::new(),
    })
}
