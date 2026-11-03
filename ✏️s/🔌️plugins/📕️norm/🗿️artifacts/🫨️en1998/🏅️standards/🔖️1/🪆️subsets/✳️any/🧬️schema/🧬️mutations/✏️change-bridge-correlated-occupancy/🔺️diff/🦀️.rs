//! ✏️ `change-bridge-correlated-occupancy` diff: patches `correlated_occupancy` of the addressed bridge row.

use crate::change_bridge_correlated_occupancy::ChangeBridgeCorrelatedOccupancy;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeCorrelatedOccupancy, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.correlated_occupancy == payload.new_correlated_occupancy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.id, En1998BridgePatch { correlated_occupancy: Some(payload.new_correlated_occupancy), ..Default::default() }), ..Default::default() })
}
