use super::ChangeMemberStirrupSpacing;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeMemberStirrupSpacing, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(s) = m.stirrups.as_ref() else { return vec![]; };
    vec![En1992Mutation::ChangeMemberStirrupSpacing(ChangeMemberStirrupSpacing { member_id: payload.member_id.clone(), new_spacing: s.spacing })]
}
