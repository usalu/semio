//! 🌀 `insert-dynamic-element` diff: inserts the dynamic element row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_dynamic_element::InsertDynamicElement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DynamicElementDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertDynamicElement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.dynamic_elements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Dynamic element {key} already exists."), [key]);
    }
    if payload.index > base.dynamic_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.dynamic_elements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { dynamic_elements: En1991DynamicElementDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
