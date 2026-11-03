//! ➕️ `insert-self-weight-element` diff: inserts the self weight element row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_self_weight_element::InsertSelfWeightElement;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SelfWeightElementDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertSelfWeightElement, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.self_weight_elements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("self weight element {key} already exists."), [key]);
    }
    if payload.index > base.self_weight_elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.self_weight_elements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_elements: En1991SelfWeightElementDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
