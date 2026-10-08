//! 🪚 `remove-member` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use super::RemoveMember;
use crate::mutations::insert_member::InsertMember;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(payload: &RemoveMember, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok(base.members.get(payload.index).map(|item| vec![En1990Mutation::InsertMember(InsertMember { index: Some(payload.index), item: item.clone() })]).unwrap_or_default())
}
