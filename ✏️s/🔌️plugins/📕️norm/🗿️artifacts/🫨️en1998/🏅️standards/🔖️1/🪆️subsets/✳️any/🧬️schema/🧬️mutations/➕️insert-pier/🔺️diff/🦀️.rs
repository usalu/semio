//! ➕️ `insert-pier` diff: inserts the pier row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_pier::InsertPier;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertPier, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.piers.iter().any(|existing| existing.id == payload.pier.id) {
        let key = payload.pier.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("pier {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.piers.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::insertion(index, payload.pier.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the pier list; inserted at {index}.", payload.index.unwrap_or(index)))
}
