//! ↩️ `change-plated-half-width` inverse: restores the previous `half_width`.

use crate::change_plated_half_width::ChangePlatedHalfWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedHalfWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_half_width::ChangePlatedHalfWidth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedHalfWidth { id: payload.id.clone(), new_half_width: row.half_width })],
        None => Vec::new(),
    })
}
