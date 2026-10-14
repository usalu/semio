use crate::insert_member::InsertMember;
use crate::remove_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertMember, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.members.len());
    vec![Op::from(remove_member::RemoveMember { index: at })]

    })())
}
