//! ↩️ `change-members` inverse.

use crate::change_members::ChangeMembers;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeMembers, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_members::ChangeMembers> {
    Ok((|| {
    vec![Op::from(ChangeMembers { members: base.members.clone() })]

    })())
}
