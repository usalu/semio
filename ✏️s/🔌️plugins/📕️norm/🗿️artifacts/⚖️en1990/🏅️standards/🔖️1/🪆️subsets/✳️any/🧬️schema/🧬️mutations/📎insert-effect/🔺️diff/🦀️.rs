//! 🔺️ `insert-effect` diff — inserts the member effect at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning. Member effects carry no identity of their own: one member may take the same action along several load
//! paths, so a repeated member/action pair is a further effect, not a duplicate.

use super::InsertEffect;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &InsertEffect, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    let mut next = base.effects.clone();
    let index = payload.index.min(next.len());
    next.insert(index, payload.item.clone());
    let outcome = MutationOutcome::new(En1990Diff { effects: Some(next), ..En1990Diff::default() });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the member effect list; inserted at {index}.", payload.index))
}
