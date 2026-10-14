use crate::insert_cold_formed_member::InsertColdFormedMember;
use crate::remove_cold_formed_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertColdFormedMember, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_cold_formed_member::RemoveColdFormedMember> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.cold_formed_members.len());
    vec![Op::from(remove_cold_formed_member::RemoveColdFormedMember { index: at })]

    })())
}
