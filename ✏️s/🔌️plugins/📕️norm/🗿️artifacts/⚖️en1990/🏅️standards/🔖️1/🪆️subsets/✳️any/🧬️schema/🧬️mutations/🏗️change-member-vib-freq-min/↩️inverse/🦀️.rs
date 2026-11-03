//! ↩️ `change-member-vib-freq-min` inverse: restores the previous `vibration_frequency_min`.

use crate::change_member_vib_freq_min::ChangeMemberVibFreqMin;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberVibFreqMin, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_vib_freq_min::ChangeMemberVibFreqMin> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberVibFreqMin, base: &En1990Snapshot) -> Option<ChangeMemberVibFreqMin> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberVibFreqMin { id: payload.id.clone(), new_vibration_frequency_min: r0.vibration_frequency_min })
}
