use crate::mutations::change_member_effective_depth::ChangeMemberEffectiveDepth;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeMemberEffectiveDepth, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    vec![En1992Mutation::ChangeMemberEffectiveDepth(ChangeMemberEffectiveDepth { member_id: payload.member_id.clone(), new_value: m.effective_depth })]
}
