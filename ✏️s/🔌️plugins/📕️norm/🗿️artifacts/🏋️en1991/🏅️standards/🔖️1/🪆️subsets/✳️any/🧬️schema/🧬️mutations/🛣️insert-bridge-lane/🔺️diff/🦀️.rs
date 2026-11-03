//! 🛣️ `insert-bridge-lane` diff: inserts the bridge lane row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bridge_lane::InsertBridgeLane;
use semio_s_artifact_norm_en1991_diff::{En1991BridgeLaneDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertBridgeLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.bridge_lanes.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("bridge lane {key} already exists."), [key]);
    }
    if payload.index > base.bridge_lanes.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.bridge_lanes.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lanes: En1991BridgeLaneDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
