use crate::insert_member::InsertMember;
use crate::remove_member;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertMember, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> {
    Ok((|| {
    let at = payload.index.min(base.members.len());
    vec![Op::from(remove_member::RemoveMember { index: at })]

    })())
}
