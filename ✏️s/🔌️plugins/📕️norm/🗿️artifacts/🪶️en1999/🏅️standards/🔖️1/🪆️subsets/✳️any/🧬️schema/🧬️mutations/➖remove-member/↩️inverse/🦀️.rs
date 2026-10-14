//! ↩️ `remove-member` inverse.

use crate::remove_member::RemoveMember;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::add_member;

pub fn inverse<Op>(payload: &RemoveMember, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_member::AddMember> {
    Ok((|| {
    if let Some((index, member)) = base.members.iter().enumerate().find(|(_, m)| m.id == payload.id) {
        vec![Op::from(add_member::AddMember { index: index as u32, member: member.clone() })]
    } else { Vec::new() }

    })())
}
