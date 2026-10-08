//! ➕ `insert-permanent` diff — inserts the row at its position; a position past the collection's end inserts it last as a
//! `mutation.clamped` warning and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertPermanent;
use crate::diff::{En1990Diff, En1990PermanentDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &InsertPermanent, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.permanents.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return MutationOutcome::fatal("mutation.duplicate-id", format!("The permanent action '{key}' already exists."), [key]);
    }
    let index = payload.index.min(base.permanents.len());
    let outcome = MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta::insertion(&base.permanents, index, payload.item.clone()), ..En1990Diff::default() });
    if index == payload.index {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the permanent action list; inserted at {index}.", payload.index))
}
