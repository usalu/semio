//! ↩️ `change-member-strain-rate-shift` inverse: restores the previous `strain_rate_shift`.

use crate::change_member_strain_rate_shift::ChangeMemberStrainRateShift;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStrainRateShift, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_strain_rate_shift::ChangeMemberStrainRateShift> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberStrainRateShift { id: payload.id.clone(), new_strain_rate_shift: row.strain_rate_shift })],
        None => Vec::new(),
    })
}
