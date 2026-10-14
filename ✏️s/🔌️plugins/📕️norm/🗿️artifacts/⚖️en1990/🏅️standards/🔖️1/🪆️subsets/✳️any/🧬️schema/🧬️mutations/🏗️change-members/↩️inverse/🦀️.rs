//! 🏗️ `change-members` inverse — removes the collection's new rows, then inserts the base rows in order; the rows are stored in reverse, as the store replays inverses reversed.

use crate::change_members::ChangeMembers;
use crate::insert_member::InsertMember;
use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeMembers, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> + From<crate::insert_member::InsertMember> {
    let removed = mutation.new_members.iter().map(|_| Op::from(RemoveMember { index: 0 }));
    let inserted = base.members.iter().cloned().enumerate().map(|(index, item)| Op::from(InsertMember { index: Some(index), item }));
    Ok(removed.chain(inserted).rev().collect())
}
