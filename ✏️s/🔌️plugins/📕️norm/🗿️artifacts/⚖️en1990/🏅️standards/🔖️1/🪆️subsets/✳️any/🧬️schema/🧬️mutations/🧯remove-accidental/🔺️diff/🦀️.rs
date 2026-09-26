use super::RemoveAccidental;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &RemoveAccidental, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.accidentals.len() {
        return MutationOutcome::fatal("mutation.invariant", format!("accidentals index out of range"), Vec::<String>::new());
    }
    let mut next = base.accidentals.clone();
    next.remove(payload.index);
    MutationOutcome::new(En1990Diff { accidentals: Some(next), ..En1990Diff::default() })
}
