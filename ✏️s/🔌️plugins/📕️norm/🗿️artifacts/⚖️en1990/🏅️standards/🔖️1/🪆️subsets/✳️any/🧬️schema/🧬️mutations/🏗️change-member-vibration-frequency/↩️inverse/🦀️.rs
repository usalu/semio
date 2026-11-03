//! ↩️ `change-member-vibration-frequency` inverse: restores the previous `vibration_frequency`.

use crate::change_member_vibration_frequency::ChangeMemberVibrationFrequency;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberVibrationFrequency, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_vibration_frequency::ChangeMemberVibrationFrequency> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberVibrationFrequency, base: &En1990Snapshot) -> Option<ChangeMemberVibrationFrequency> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberVibrationFrequency { id: payload.id.clone(), new_vibration_frequency: r0.vibration_frequency })
}
