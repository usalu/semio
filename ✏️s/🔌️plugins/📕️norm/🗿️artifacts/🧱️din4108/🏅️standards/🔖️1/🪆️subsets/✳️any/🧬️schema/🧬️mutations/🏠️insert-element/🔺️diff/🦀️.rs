//! 🏠️ `insert-element` diff — inserts the row at its position; a position past the list's end inserts it last as a
//! `mutation.clamped` warning, and an id the document already holds is a `mutation.duplicate-id`.

use super::InsertElement;
use crate::diff::{Din4108Diff, Din4108ElementDelta};
use crate::Din4108Snapshot;

pub fn diff(payload: &InsertElement, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.elements.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An envelope element with id '{}' already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.elements.len());
    let outcome = protocol::MutationOutcome::new(Din4108Diff { elements: Din4108ElementDelta::insertion(index, payload.element.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the envelope element list; inserted at {index}.", payload.index.unwrap_or(index)))
}
