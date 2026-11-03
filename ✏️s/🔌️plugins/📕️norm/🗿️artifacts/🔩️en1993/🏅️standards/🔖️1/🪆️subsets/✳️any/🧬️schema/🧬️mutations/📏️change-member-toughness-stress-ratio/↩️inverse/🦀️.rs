//! ↩️ `change-member-toughness-stress-ratio` inverse: restores the previous `toughness_stress_ratio`.

use crate::change_member_toughness_stress_ratio::ChangeMemberToughnessStressRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberToughnessStressRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_toughness_stress_ratio::ChangeMemberToughnessStressRatio> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberToughnessStressRatio { id: payload.id.clone(), new_toughness_stress_ratio: row.toughness_stress_ratio })],
        None => Vec::new(),
    })
}
