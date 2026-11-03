//! 🌉 `insert-bridge` diff: inserts the bridge row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bridge::InsertBridge;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertBridge, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.bridges.iter().any(|existing| existing.id == payload.bridge.id) {
        let key = payload.bridge.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("bridge {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.bridges.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::insertion(index, payload.bridge.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the bridge list; inserted at {index}.", payload.index.unwrap_or(index)))
}
