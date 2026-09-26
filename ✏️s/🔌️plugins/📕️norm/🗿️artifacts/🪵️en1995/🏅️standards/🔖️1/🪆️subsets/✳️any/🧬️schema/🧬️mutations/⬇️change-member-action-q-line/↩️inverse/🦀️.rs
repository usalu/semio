use super::ChangeMemberActionQLine;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberActionQLine, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };
    vec![En1995Mutation::ChangeMemberActionQLine(ChangeMemberActionQLine { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: item.q_line_n_per_m })]
}
