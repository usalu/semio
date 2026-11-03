//! ↩️ `change-plated-hogging` inverse: restores the previous `hogging`.

use crate::change_plated_hogging::ChangePlatedHogging;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedHogging, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_hogging::ChangePlatedHogging> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedHogging { id: payload.id.clone(), new_hogging: row.hogging })],
        None => Vec::new(),
    })
}
