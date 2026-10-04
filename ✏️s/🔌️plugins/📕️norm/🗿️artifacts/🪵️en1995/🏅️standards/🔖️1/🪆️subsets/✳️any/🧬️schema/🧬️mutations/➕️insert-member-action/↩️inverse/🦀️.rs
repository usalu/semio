use super::InsertMemberAction;
use crate::mutations::{remove_member_action, En1995Mutation};
use crate::En1995Snapshot;
pub fn inverse(payload: &InsertMemberAction, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };
    let at = payload.index.min(item.actions.len());
    vec![En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { member_id: payload.member_id.clone(), index: at })]

    })())
}
