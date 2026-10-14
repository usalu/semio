use crate::insert_member::InsertMember;
use crate::remove_member::RemoveMember;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertMember, _base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> {
    Ok((|| {
    vec![Op::from(RemoveMember { member_id: payload.member.id.clone() })]

    })())
}
