use crate::remove_cold_formed_member::RemoveColdFormedMember;
use crate::insert_cold_formed_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveColdFormedMember, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_cold_formed_member::InsertColdFormedMember> {
    Ok((|| {
    if payload.index >= base.cold_formed_members.len() { return Vec::new(); }
    vec![Op::from(insert_cold_formed_member::InsertColdFormedMember { index: Some(payload.index), cold_formed_member: base.cold_formed_members[payload.index].clone() })]

    })())
}
