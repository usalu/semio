//! ➕️ `insert-seismic-slope` diff: inserts the seismic_slope row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_seismic_slope::InsertSeismicSlope;
use semio_s_artifact_norm_en1998_diff::{En1998SlopeDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertSeismicSlope, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.slopes.iter().any(|existing| existing.id == payload.seismic_slope.id) {
        let key = payload.seismic_slope.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("seismic_slope {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.slopes.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { slopes: En1998SlopeDelta::insertion(index, payload.seismic_slope.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the seismic_slope list; inserted at {index}.", payload.index.unwrap_or(index)))
}
