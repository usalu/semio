//! ↩️ `change-plated-compression-stress` inverse: restores the previous `compression_stress`.

use crate::change_plated_compression_stress::ChangePlatedCompressionStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedCompressionStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_compression_stress::ChangePlatedCompressionStress> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedCompressionStress { id: payload.id.clone(), new_compression_stress: row.compression_stress })],
        None => Vec::new(),
    })
}
