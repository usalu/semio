//! ➕️ `insert-load-case` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertLoadCase;
use crate::diff::{En1993Diff, En1993LoadCaseDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertLoadCase, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.load_cases.iter().any(|existing| existing.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Load case id {} already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.load_cases.len());
    protocol::MutationOutcome::new(En1993Diff { load_cases: En1993LoadCaseDelta::insertion(index, payload.load_case.clone()), ..Default::default() })
}
