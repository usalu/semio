//! 🪨 `insert-foundation` diff: inserts the foundation row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_foundation::InsertFoundation;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.foundations.iter().any(|existing| existing.id == payload.foundation.id) {
        let key = payload.foundation.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("foundation {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.foundations.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::insertion(index, payload.foundation.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the foundation list; inserted at {index}.", payload.index.unwrap_or(index)))
}
