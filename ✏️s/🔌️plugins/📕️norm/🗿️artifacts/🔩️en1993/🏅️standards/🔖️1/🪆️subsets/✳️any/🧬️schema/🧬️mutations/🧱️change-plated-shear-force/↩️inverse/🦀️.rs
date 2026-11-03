//! ↩️ `change-plated-shear-force` inverse: restores the previous `shear_force`.

use crate::change_plated_shear_force::ChangePlatedShearForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedShearForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_shear_force::ChangePlatedShearForce> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedShearForce { id: payload.id.clone(), new_shear_force: row.shear_force })],
        None => Vec::new(),
    })
}
