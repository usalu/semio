//! ↩️ `change-weld-shear-force` inverse: restores the previous `shear_force`.

use crate::change_weld_shear_force::ChangeWeldShearForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldShearForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_shear_force::ChangeWeldShearForce> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldShearForce { id: payload.id.clone(), new_shear_force: row.shear_force })],
        None => Vec::new(),
    })
}
