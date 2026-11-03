//! ➕️ `insert-diaphragm` diff: inserts the diaphragm row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_diaphragm::InsertDiaphragm;
use semio_s_artifact_norm_en1998_diff::{En1998DiaphragmDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertDiaphragm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.diaphragms.iter().any(|existing| existing.id == payload.diaphragm.id) {
        let key = payload.diaphragm.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("diaphragm {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.diaphragms.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { diaphragms: En1998DiaphragmDelta::insertion(index, payload.diaphragm.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the diaphragm list; inserted at {index}.", payload.index.unwrap_or(index)))
}
