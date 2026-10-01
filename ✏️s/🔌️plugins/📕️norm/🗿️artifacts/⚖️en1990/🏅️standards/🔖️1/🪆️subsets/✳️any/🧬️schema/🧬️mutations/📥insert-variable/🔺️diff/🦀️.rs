//! 🔺️ `insert-variable` diff — inserts the variable action at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertVariable;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &InsertVariable, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.variables.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return MutationOutcome::fatal("mutation.duplicate-id", format!("The variable action '{key}' already exists."), [key]);
    }
    let mut next = base.variables.clone();
    let index = payload.index.min(next.len());
    next.insert(index, payload.item.clone());
    let outcome = MutationOutcome::new(En1990Diff { variables: Some(next), ..En1990Diff::default() });
    if index == payload.index {
        return outcome;
    }
    outcome.warn("mutation.clamped", format!("Position {} is past the end of the variable action list; inserted at {index}.", payload.index))
}
