//! ↩️ `change-fatigue-member-id` inverse: restores the previous `member_id`.

use crate::change_fatigue_member_id::ChangeFatigueMemberId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueMemberId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_member_id::ChangeFatigueMemberId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueMemberId, base: &En1990Snapshot) -> Option<ChangeFatigueMemberId> {
    let r0 = base.fatigue_spectra.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueMemberId { id: payload.id.clone(), new_member_id: r0.member_id.clone() })
}
