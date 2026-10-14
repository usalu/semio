//! ↩️ `change-member-my-ed` inverse.

use crate::change_member_m_y_ed::ChangeMemberMYEd;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMYEd, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_m_y_ed::ChangeMemberMYEd> {
    Ok((|| {
    let v = base.members.iter().find(|m| m.id == payload.member_id)
        .and_then(|m| m.actions.iter().find(|a| a.id == payload.action_id).or_else(|| m.actions.first()))
        .map(|a| a.m_y_k).unwrap_or(0.0);
    vec![Op::from(ChangeMemberMYEd { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_m_y_k: v })]

    })())
}
