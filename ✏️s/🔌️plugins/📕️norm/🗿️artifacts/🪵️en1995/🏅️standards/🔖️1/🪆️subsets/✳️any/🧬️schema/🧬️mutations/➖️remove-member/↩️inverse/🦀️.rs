use crate::remove_member::RemoveMember;
use crate::insert_member;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveMember, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member::InsertMember> {
    Ok((|| {
    if payload.index >= base.members.len() { return Vec::new(); }
    vec![Op::from(insert_member::InsertMember { index: payload.index, member: base.members[payload.index].clone() })]

    })())
}
