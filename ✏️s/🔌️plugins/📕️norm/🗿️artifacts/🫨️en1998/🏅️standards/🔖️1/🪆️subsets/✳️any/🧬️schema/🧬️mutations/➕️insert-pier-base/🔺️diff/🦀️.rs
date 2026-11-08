//! ➕️ `insert-pier-base` diff: inserts the pier_base row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_pier_base::InsertPierBase;
use semio_s_artifact_norm_en1998_diff::{En1998PierBaseDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertPierBase, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.pier_bases.iter().any(|existing| existing.id == payload.pier_base.id) {
        let key = payload.pier_base.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("pier_base {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.pier_bases.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { pier_bases: En1998PierBaseDelta::insertion(index, payload.pier_base.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the pier_base list; inserted at {index}.", payload.index.unwrap_or(index)))
}
