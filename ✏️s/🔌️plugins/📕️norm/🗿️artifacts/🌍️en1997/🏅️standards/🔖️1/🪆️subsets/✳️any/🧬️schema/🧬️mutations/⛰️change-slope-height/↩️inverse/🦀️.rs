//! ↩️ `change-slope-height` inverse: restores the previous `height`.

use crate::change_slope_height::ChangeSlopeHeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeHeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_height::ChangeSlopeHeight> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeHeight { id: payload.id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
