use super::InsertMember;
use crate::mutations::remove_member::RemoveMember;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &InsertMember, _base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::RemoveMember(RemoveMember { member_id: payload.member.id.clone() })]
}
