use crate::change_member_action_q_line::ChangeMemberActionQLine;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeMemberActionQLine, base: &En1995Snapshot) -> Option<crate::change_member_action_q_line::ChangeMemberActionQLine> {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return None; };
    Some(ChangeMemberActionQLine { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: item.q_line_n_per_m })
}
