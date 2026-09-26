use super::ChangeMemberFireRating;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeMemberFireRating, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(f) = m.fire.as_ref() else { return vec![]; };
    vec![En1992Mutation::ChangeMemberFireRating(ChangeMemberFireRating { member_id: payload.member_id.clone(), new_rating: f.rating })]
}
