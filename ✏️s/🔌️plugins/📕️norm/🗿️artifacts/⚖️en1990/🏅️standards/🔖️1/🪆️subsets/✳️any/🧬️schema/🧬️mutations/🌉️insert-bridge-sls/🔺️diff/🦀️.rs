//! 🌉 `insert-bridge-sls` diff: inserts the bridge serviceability row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bridge_sls::InsertBridgeSls;
use semio_s_artifact_norm_en1990_diff::{En1990BridgeSlsDelta, En1990Diff};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertBridgeSls, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.bridge_sls.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge serviceability {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.bridge_sls.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { bridge_sls: En1990BridgeSlsDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the bridge serviceability list; inserted at {index}.", payload.index.unwrap_or(index)))
}
