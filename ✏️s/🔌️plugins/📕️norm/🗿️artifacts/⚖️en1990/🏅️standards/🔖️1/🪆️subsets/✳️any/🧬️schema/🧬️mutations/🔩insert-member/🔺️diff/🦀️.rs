use super::InsertMember;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &InsertMember, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    let mut next = base.members.clone();
    let i = payload.index.min(next.len());
    next.insert(i, payload.item.clone());
    MutationOutcome::new(En1990Diff { members: Some(next), ..En1990Diff::default() })
}
