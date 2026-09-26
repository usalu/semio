use crate::mutations::change_action_n_ed::ChangeActionNEd;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeActionNEd, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(a) = m.actions.iter().find(|a| a.id == payload.action_id) else { return vec![]; };
    vec![En1992Mutation::ChangeActionNEd(ChangeActionNEd { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: a.n_k })]
}
