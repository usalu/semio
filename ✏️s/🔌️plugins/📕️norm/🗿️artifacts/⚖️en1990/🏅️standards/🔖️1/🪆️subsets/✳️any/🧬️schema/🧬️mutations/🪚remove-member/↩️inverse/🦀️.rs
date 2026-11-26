//! 🪚 `remove-member` inverse — inserts the removed row back at its position; an absent row leaves nothing to restore.

use crate::remove_member::RemoveMember;
use crate::insert_member::InsertMember;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveMember, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member::InsertMember> {
    Ok(match previous(payload, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(payload: &RemoveMember, base: &En1990Snapshot) -> Option<crate::insert_member::InsertMember> {
    base.members.get(payload.index).map(|item| InsertMember { index: Some(payload.index), item: item.clone() })
}
