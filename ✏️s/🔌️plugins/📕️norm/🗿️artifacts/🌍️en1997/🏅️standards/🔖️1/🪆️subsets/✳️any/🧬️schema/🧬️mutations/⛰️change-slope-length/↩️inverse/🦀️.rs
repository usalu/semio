//! ↩️ `change-slope-length` inverse: restores the previous `length`.

use crate::change_slope_length::ChangeSlopeLength;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeLength, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_length::ChangeSlopeLength> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeLength { id: payload.id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
