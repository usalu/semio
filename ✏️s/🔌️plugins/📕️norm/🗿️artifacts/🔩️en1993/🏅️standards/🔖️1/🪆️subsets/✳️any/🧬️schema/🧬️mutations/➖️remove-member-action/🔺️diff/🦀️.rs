use super::RemoveMemberAction;
use crate::diff::En1993MemberActionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.member_actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.member_actions.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { member_actions: Some(En1993MemberActionList { values }), ..Default::default() })
}
