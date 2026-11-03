use crate::insert_frame_member_basis::InsertFrameMemberBasis;
use crate::remove_frame_member_basis;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFrameMemberBasis, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_frame_member_basis::RemoveFrameMemberBasis> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.frame_member_bases.len());
    vec![Op::from(remove_frame_member_basis::RemoveFrameMemberBasis { index: at })]

    })())
}
