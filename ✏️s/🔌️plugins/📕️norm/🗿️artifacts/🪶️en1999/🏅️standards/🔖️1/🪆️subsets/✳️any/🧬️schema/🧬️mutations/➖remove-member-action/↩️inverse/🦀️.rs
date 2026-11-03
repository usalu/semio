//! ↩️ `remove-member-action` inverse.

use crate::remove_member_action::RemoveMemberAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_member_action;

pub fn inverse<Op>(payload: &RemoveMemberAction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member_action::InsertMemberAction> {
    Ok((|| {
    match base.members.iter().find(|row| row.id == payload.member_id).and_then(|parent| parent.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(insert_member_action::InsertMemberAction { member_id: payload.member_id.clone(), index: index as u32, action: row.clone() })],
        None => Vec::new(),
    }
    })())
}
