//! 🌉 `insert-wind-bridge` diff: inserts the bridge wind case row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_wind_bridge::InsertWindBridge;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertWindBridge, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.wind_bridges.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bridge wind case {key} already exists."), [key]);
    }
    if payload.index > base.wind_bridges.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.wind_bridges.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
