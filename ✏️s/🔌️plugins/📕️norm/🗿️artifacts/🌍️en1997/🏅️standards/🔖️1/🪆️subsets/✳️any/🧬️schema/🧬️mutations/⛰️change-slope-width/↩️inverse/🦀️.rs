//! ↩️ `change-slope-width` inverse: restores the previous `width`.

use crate::change_slope_width::ChangeSlopeWidth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeWidth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_width::ChangeSlopeWidth> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeWidth { id: payload.id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
