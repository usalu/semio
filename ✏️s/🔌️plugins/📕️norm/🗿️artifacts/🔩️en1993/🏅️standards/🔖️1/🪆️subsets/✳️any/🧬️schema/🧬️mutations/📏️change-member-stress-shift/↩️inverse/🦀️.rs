//! ↩️ `change-member-stress-shift` inverse: restores the previous `stress_shift`.

use crate::change_member_stress_shift::ChangeMemberStressShift;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStressShift, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_stress_shift::ChangeMemberStressShift> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberStressShift { id: payload.id.clone(), new_stress_shift: row.stress_shift })],
        None => Vec::new(),
    })
}
