//! 🫙 `insert-silo` diff: inserts the silo row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_silo::InsertSilo;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SiloDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.silos.iter().any(|existing| existing.id == payload.silo.id) {
        let key = payload.silo.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("silo {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.silos.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::insertion(index, payload.silo.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the silo list; inserted at {index}.", payload.index.unwrap_or(index)))
}
