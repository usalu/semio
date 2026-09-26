use super::InsertAccidental;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &InsertAccidental, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    let mut next = base.accidentals.clone();
    let i = payload.index.min(next.len());
    next.insert(i, payload.item.clone());
    MutationOutcome::new(En1990Diff { accidentals: Some(next), ..En1990Diff::default() })
}
