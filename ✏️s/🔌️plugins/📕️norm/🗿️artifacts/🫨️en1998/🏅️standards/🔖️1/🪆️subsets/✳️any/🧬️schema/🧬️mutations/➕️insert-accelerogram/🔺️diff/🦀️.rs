//! ➕️ `insert-accelerogram` diff: inserts the accelerogram row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_accelerogram::InsertAccelerogram;
use semio_s_artifact_norm_en1998_diff::{En1998AccelerogramDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertAccelerogram, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.accelerograms.iter().any(|existing| existing.id == payload.accelerogram.id) {
        let key = payload.accelerogram.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("accelerogram {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.accelerograms.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { accelerograms: En1998AccelerogramDelta::insertion(index, payload.accelerogram.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the accelerogram list; inserted at {index}.", payload.index.unwrap_or(index)))
}
