//! ↩️ `change-plated-shear-stress` inverse: restores the previous `shear_stress`.

use crate::change_plated_shear_stress::ChangePlatedShearStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedShearStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_shear_stress::ChangePlatedShearStress> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedShearStress { id: payload.id.clone(), new_shear_stress: row.shear_stress })],
        None => Vec::new(),
    })
}
