use super::RemoveMember;
use crate::mutations::insert_member::InsertMember;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &RemoveMember, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some((index, member)) = base.members.iter().enumerate().find(|(_, m)| m.id == payload.member_id) else { return vec![]; };
    vec![En1992Mutation::InsertMember(InsertMember { index, member: member.clone() })]
}
