use super::InsertMember;
use crate::{En1995Diff, En1995Snapshot};
use crate::diff::{En1995MemberDelta};
pub fn diff(payload: &InsertMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.members.iter().any(|existing| existing.id == payload.member.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} already exists.", payload.member.id), [payload.member.id.clone()]);
    }
    let at = payload.index.min(base.members.len());
    protocol::MutationOutcome::new(En1995Diff { members: En1995MemberDelta::insertion(&base.members, at, payload.member.clone()), ..Default::default() })
}
