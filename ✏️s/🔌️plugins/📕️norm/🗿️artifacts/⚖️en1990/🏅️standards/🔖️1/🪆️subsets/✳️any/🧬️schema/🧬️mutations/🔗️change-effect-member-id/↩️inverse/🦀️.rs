//! ↩️ `change-effect-member-id` inverse: restores the previous `member_id`.

use crate::change_effect_member_id::ChangeEffectMemberId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeEffectMemberId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_effect_member_id::ChangeEffectMemberId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeEffectMemberId, base: &En1990Snapshot) -> Option<ChangeEffectMemberId> {
    let r0 = base.effects.get(payload.index)?;
    Some(ChangeEffectMemberId { index: payload.index, new_member_id: r0.member_id.clone() })
}
