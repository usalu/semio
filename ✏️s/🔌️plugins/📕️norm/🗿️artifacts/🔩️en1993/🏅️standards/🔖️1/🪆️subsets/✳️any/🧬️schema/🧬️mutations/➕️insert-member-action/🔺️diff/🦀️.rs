use super::InsertMemberAction;
use crate::diff::En1993MemberActionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.member_actions.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.member_action.clone());
    protocol::MutationOutcome::new(En1993Diff { member_actions: Some(En1993MemberActionList { values }), ..Default::default() })
}
