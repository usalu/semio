//! ↩️ `change-cf-shear-force` inverse: restores the previous `shear_force`.

use crate::change_cf_shear_force::ChangeCfShearForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfShearForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_shear_force::ChangeCfShearForce> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfShearForce { id: payload.id.clone(), new_shear_force: row.shear_force })],
        None => Vec::new(),
    })
}
