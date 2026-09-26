use crate::mutations::change_member_width::ChangeMemberWidth;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeMemberWidth, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    vec![En1992Mutation::ChangeMemberWidth(ChangeMemberWidth { member_id: payload.member_id.clone(), new_value: m.width })]
}
