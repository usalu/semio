//! ↩️ `change-plated-corrugation-flat` inverse: restores the previous `corrugation_flat`.

use crate::change_plated_corrugation_flat::ChangePlatedCorrugationFlat;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedCorrugationFlat, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_corrugation_flat::ChangePlatedCorrugationFlat> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedCorrugationFlat { id: payload.id.clone(), new_corrugation_flat: row.corrugation_flat })],
        None => Vec::new(),
    })
}
