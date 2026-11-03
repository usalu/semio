//! ↩️ `change-plated-corrugation-projection` inverse: restores the previous `corrugation_projection`.

use crate::change_plated_corrugation_projection::ChangePlatedCorrugationProjection;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedCorrugationProjection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_corrugation_projection::ChangePlatedCorrugationProjection> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedCorrugationProjection { id: payload.id.clone(), new_corrugation_projection: row.corrugation_projection })],
        None => Vec::new(),
    })
}
