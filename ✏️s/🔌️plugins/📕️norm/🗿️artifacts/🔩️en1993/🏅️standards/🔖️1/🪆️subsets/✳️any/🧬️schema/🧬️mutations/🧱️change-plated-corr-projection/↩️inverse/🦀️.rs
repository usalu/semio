//! ↩️ `change-plated-corr-projection` inverse: restores the previous `corrugation_projection`.

use crate::change_plated_corr_projection::ChangePlatedCorrProjection;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedCorrProjection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_corr_projection::ChangePlatedCorrProjection> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedCorrProjection { id: payload.id.clone(), new_corrugation_projection: row.corrugation_projection })],
        None => Vec::new(),
    })
}
