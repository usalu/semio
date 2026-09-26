use super::InsertEffect;
use crate::diff::En1990Diff; use crate::En1990Snapshot; use protocol::MutationOutcome;
pub fn diff(payload: &InsertEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    let mut next = base.effects.clone();
    let i = payload.index.min(next.len());
    next.insert(i, payload.item.clone());
    MutationOutcome::new(En1990Diff { effects: Some(next), ..En1990Diff::default() })
}
