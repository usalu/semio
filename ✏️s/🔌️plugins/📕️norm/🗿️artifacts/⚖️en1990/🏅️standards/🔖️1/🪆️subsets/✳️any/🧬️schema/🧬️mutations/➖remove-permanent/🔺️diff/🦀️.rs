use super::RemovePermanent;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemovePermanent, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.permanents.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("permanents index out of range"), Vec::<String>::new());
    }
    let mut next = base.permanents.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { permanents: Some(next), ..En1990Diff::default() })
}
