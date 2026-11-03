//! ↩️ `change-member-radiation-shift` inverse: restores the previous `radiation_shift`.

use crate::change_member_radiation_shift::ChangeMemberRadiationShift;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRadiationShift, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_radiation_shift::ChangeMemberRadiationShift> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberRadiationShift { id: payload.id.clone(), new_radiation_shift: row.radiation_shift })],
        None => Vec::new(),
    })
}
