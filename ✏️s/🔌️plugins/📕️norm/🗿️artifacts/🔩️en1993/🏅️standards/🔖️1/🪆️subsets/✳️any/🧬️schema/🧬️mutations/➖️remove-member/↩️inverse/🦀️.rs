use super::RemoveMember;
use crate::mutations::{insert_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveMember, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.members.len() { return Vec::new(); }
    vec![En1993Mutation::InsertMember(insert_member::InsertMember { index: Some(payload.index), member: base.members[payload.index].clone() })]

    })())
}
