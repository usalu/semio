//! 🔩 `insert-member` inverse — removes the row at the position the insert landed on.

use super::InsertMember;
use crate::mutations::remove_member::RemoveMember;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &InsertMember, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(vec![En1990Mutation::RemoveMember(RemoveMember { index: payload.index.min(base.members.len()) })])
}
