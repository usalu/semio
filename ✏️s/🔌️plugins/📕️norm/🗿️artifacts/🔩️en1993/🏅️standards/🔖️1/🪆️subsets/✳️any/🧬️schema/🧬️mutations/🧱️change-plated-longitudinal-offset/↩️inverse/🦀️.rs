//! ↩️ `change-plated-longitudinal-offset` inverse: restores the previous `longitudinal_offset`.

use crate::change_plated_longitudinal_offset::ChangePlatedLongitudinalOffset;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedLongitudinalOffset, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_longitudinal_offset::ChangePlatedLongitudinalOffset> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedLongitudinalOffset { id: payload.id.clone(), new_longitudinal_offset: row.longitudinal_offset })],
        None => Vec::new(),
    })
}
