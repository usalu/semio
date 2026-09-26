use super::ChangeMemberSpan;
use crate::diff::En1995MemberList;
use crate::{En1995Diff, En1995Snapshot};
pub fn diff(payload: &ChangeMemberSpan, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(idx) = base.members.iter().position(|item| item.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member id.", vec![payload.member_id.clone()]);
    };
    let mut members = base.members.clone();
    members[idx].span_m = payload.new_value;
    protocol::MutationOutcome::new(En1995Diff { members: Some(En1995MemberList { values: members }), ..Default::default() })
}
