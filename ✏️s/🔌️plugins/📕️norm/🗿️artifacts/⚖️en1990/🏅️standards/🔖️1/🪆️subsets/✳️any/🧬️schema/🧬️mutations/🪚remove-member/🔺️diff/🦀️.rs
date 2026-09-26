use super::RemoveMember;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemoveMember, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.members.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("members index out of range"), Vec::<String>::new());
    }
    let mut next = base.members.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { members: Some(next), ..En1990Diff::default() })
}
