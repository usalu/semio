use super::RemoveMember;
use crate::mutations::{insert_member, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &RemoveMember, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.members.len() { return Vec::new(); }
    vec![En1995Mutation::InsertMember(insert_member::InsertMember { index: payload.index, member: base.members[payload.index].clone() })]

    })())
}
