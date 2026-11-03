use crate::remove_built_up_member::RemoveBuiltUpMember;
use crate::insert_built_up_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveBuiltUpMember, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_built_up_member::InsertBuiltUpMember> {
    Ok((|| {
    if payload.index >= base.built_up_members.len() { return Vec::new(); }
    vec![Op::from(insert_built_up_member::InsertBuiltUpMember { index: Some(payload.index), built_up_member: base.built_up_members[payload.index].clone() })]

    })())
}
