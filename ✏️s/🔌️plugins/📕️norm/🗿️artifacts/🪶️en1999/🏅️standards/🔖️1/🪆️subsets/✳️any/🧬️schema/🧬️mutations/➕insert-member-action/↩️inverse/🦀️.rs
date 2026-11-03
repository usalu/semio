//! ↩️ `insert-member-action` inverse.

use crate::insert_member_action::InsertMemberAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_member_action;

pub fn inverse<Op>(payload: &InsertMemberAction, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member_action::RemoveMemberAction> {
    Ok((|| {
    vec![Op::from(remove_member_action::RemoveMemberAction { member_id: payload.member_id.clone(), id: payload.action.id.clone() })]
    })())
}
