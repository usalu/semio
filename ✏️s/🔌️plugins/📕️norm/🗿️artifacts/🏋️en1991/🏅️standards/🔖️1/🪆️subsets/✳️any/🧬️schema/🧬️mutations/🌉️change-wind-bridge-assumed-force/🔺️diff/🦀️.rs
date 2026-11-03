//! 🌉 `change-wind-bridge-assumed-force` diff: patches `assumed_force` of the addressed bridge wind case row.

use crate::change_wind_bridge_assumed_force::ChangeWindBridgeAssumedForce;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta, En1991WindBridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindBridgeAssumedForce, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge wind case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_force == payload.new_assumed_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge wind case already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::modification(&payload.id, En1991WindBridgePatch { assumed_force: Some(payload.new_assumed_force), ..Default::default() }), ..Default::default() })
}
