//! 🏗️ `change-members` inverse — removes the collection's new rows back to front, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use super::ChangeMembers;
use crate::mutations::insert_member::InsertMember;
use crate::mutations::remove_member::RemoveMember;
use crate::{En1990Mutation, En1990Snapshot};

pub fn inverse(mutation: &ChangeMembers, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    let removed = (0..mutation.new_members.len()).rev().map(|index| En1990Mutation::RemoveMember(RemoveMember { index }));
    let inserted = base.members.iter().cloned().enumerate().map(|(index, item)| En1990Mutation::InsertMember(InsertMember { index, item }));
    Ok(removed.chain(inserted).rev().collect())
}
