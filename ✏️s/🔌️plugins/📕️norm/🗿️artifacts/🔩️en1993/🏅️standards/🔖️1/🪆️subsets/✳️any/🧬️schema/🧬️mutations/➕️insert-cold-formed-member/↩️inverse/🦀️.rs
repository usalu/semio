use super::InsertColdFormedMember;
use crate::mutations::{remove_cold_formed_member, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.cold_formed_members.len());
    vec![En1993Mutation::RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember { index: at })]

    })())
}
