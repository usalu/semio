//! ↩️ `change-member-n-ed` inverse.

use crate::mutations::change_member_n_ed::ChangeMemberNEd;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeMemberNEd, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let n = base.members.iter().find(|m| m.id == payload.member_id)
        .and_then(|m| m.actions.iter().find(|a| a.id == payload.action_id).or_else(|| m.actions.first()))
        .map(|a| a.n_k).unwrap_or(0.0);
    vec![En1999Mutation::ChangeMemberNEd(ChangeMemberNEd { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_n_k: n })]
}
