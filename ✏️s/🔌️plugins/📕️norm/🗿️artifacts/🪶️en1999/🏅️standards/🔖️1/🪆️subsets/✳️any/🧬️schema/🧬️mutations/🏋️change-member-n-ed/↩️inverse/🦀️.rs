//! ↩️ `change-member-n-ed` inverse.

use crate::change_member_n_ed::ChangeMemberNEd;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberNEd, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_n_ed::ChangeMemberNEd> {
    Ok((|| {
    let n = base.members.iter().find(|m| m.id == payload.member_id)
        .and_then(|m| m.actions.iter().find(|a| a.id == payload.action_id).or_else(|| m.actions.first()))
        .map(|a| a.n_k).unwrap_or(0.0);
    vec![Op::from(ChangeMemberNEd { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_n_k: n })]

    })())
}
