//! 🚆 `insert-derailment` diff: inserts the derailment case row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_derailment::InsertDerailment;
use semio_s_artifact_norm_en1991_diff::{En1991DerailmentCaseDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertDerailment, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.derailment_cases.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Derailment case {key} already exists."), [key]);
    }
    if payload.index > base.derailment_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.derailment_cases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { derailment_cases: En1991DerailmentCaseDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
