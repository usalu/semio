use crate::insert_built_up_member::InsertBuiltUpMember;
use crate::remove_built_up_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertBuiltUpMember, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_built_up_member::RemoveBuiltUpMember> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.built_up_members.len());
    vec![Op::from(remove_built_up_member::RemoveBuiltUpMember { index: at })]

    })())
}
