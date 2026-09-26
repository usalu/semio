//! ↩️ `change-member-my-ed` inverse.

use crate::mutations::change_member_m_y_ed::ChangeMemberMYEd;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeMemberMYEd, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let v = base.members.iter().find(|m| m.id == payload.member_id)
        .and_then(|m| m.actions.iter().find(|a| a.id == payload.action_id).or_else(|| m.actions.first()))
        .map(|a| a.m_y_k).unwrap_or(0.0);
    vec![En1999Mutation::ChangeMemberMYEd(ChangeMemberMYEd { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_m_y_k: v })]
}
