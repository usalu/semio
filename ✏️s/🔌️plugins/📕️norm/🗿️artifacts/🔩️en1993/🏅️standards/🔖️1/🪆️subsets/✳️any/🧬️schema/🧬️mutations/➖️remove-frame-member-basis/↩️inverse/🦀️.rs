use crate::remove_frame_member_basis::RemoveFrameMemberBasis;
use crate::insert_frame_member_basis;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFrameMemberBasis, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_frame_member_basis::InsertFrameMemberBasis> {
    Ok((|| {
    if payload.index >= base.frame_member_bases.len() { return Vec::new(); }
    vec![Op::from(insert_frame_member_basis::InsertFrameMemberBasis { index: Some(payload.index), frame_member_basis: base.frame_member_bases[payload.index].clone() })]

    })())
}
