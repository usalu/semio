//! ↩️ `change-cf-flange-stress` inverse: restores the previous `flange_stress`.

use crate::change_cf_flange_stress::ChangeCfFlangeStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfFlangeStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_flange_stress::ChangeCfFlangeStress> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfFlangeStress { id: payload.id.clone(), new_flange_stress: row.flange_stress })],
        None => Vec::new(),
    })
}
