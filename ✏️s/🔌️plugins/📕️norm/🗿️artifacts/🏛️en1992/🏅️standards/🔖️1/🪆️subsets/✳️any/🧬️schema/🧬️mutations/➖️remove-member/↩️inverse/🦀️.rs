use crate::remove_member::RemoveMember;
use crate::insert_member::InsertMember;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveMember, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member::InsertMember> {
    Ok((|| {
    let Some((index, member)) = base.members.iter().enumerate().find(|(_, m)| m.id == payload.member_id) else { return vec![]; };
    vec![Op::from(InsertMember { index, member: member.clone() })]

    })())
}
