//! 🔺️ `insert-element` sparse diff: one inserted row.

use crate::insert_element::InsertElement;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ElementsRows};

pub fn diff(payload: &InsertElement, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if payload.index > base.elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Position {} is beyond the elements.", payload.index), [payload.index.to_string()]);
    }
    if base.elements.iter().any(|row| row.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Element id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    protocol::MutationOutcome::new(Din18599Diff { elements: Some(Din18599ElementsRows::insertion(payload.index, payload.element.clone())), ..Default::default() })
}
