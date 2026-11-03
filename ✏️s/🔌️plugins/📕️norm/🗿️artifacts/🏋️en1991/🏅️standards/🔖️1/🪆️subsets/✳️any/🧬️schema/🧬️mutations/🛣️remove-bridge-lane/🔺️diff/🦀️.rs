//! 🛣️ `remove-bridge-lane` diff: removes the addressed bridge lane row.

use crate::remove_bridge_lane::RemoveBridgeLane;
use semio_s_artifact_norm_en1991_diff::{En1991BridgeLaneDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveBridgeLane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.bridge_lanes.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge lane {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { bridge_lanes: En1991BridgeLaneDelta::removal(&base.bridge_lanes, index), ..Default::default() })
}
