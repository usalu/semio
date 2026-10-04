//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateMemberProperties;
use crate::mutations::remove_member;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if let Some(prior) = base.members.iter().find(|x| x.id == payload.member.id) {
        vec![En1993Mutation::UpdateMemberProperties(UpdateMemberProperties { member: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveMember(remove_member::RemoveMember { index: base.members.len() })]
    }

    })())
}
