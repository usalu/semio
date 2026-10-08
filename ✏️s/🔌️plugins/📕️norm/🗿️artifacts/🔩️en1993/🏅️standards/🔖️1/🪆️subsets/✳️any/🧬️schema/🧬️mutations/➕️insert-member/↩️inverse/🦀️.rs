use super::InsertMember;
use crate::mutations::{remove_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertMember, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.members.len());
    vec![En1993Mutation::RemoveMember(remove_member::RemoveMember { index: at })]

    })())
}
