//! ↩️ `add-member` inverse.

use crate::add_member::AddMember;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_member;

pub fn inverse<Op>(payload: &AddMember, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> {
    Ok((|| {
    vec![Op::from(remove_member::RemoveMember { id: payload.member.id.clone() })]

    })())
}
