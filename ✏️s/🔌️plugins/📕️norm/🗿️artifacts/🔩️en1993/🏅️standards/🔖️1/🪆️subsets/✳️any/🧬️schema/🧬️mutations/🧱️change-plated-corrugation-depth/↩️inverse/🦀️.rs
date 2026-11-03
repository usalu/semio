//! ↩️ `change-plated-corrugation-depth` inverse: restores the previous `corrugation_depth`.

use crate::change_plated_corrugation_depth::ChangePlatedCorrugationDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedCorrugationDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_corrugation_depth::ChangePlatedCorrugationDepth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedCorrugationDepth { id: payload.id.clone(), new_corrugation_depth: row.corrugation_depth })],
        None => Vec::new(),
    })
}
