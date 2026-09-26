use super::RemoveEffect;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemoveEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.effects.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("effects index out of range"), Vec::<String>::new());
    }
    let mut next = base.effects.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { effects: Some(next), ..En1990Diff::default() })
}
