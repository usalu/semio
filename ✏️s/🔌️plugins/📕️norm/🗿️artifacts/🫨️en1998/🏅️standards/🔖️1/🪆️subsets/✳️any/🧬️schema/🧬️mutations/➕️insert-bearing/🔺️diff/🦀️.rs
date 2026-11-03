//! ➕️ `insert-bearing` diff: inserts the bearing row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bearing::InsertBearing;
use semio_s_artifact_norm_en1998_diff::{En1998BearingDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertBearing, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.bearings.iter().any(|existing| existing.id == payload.bearing.id) {
        let key = payload.bearing.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("bearing {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.bearings.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { bearings: En1998BearingDelta::insertion(index, payload.bearing.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the bearing list; inserted at {index}.", payload.index.unwrap_or(index)))
}
