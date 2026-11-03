//! 🌉 `change-wind-bridge-ground-height` diff: patches `height_above_ground` of the addressed bridge wind case row.

use crate::change_wind_bridge_ground_height::ChangeWindBridgeGroundHeight;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991WindBridgeDelta, En1991WindBridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeWindBridgeGroundHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_height_above_ground.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.wind_bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bridge wind case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.height_above_ground == payload.new_height_above_ground {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge wind case already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { wind_bridges: En1991WindBridgeDelta::modification(&payload.id, En1991WindBridgePatch { height_above_ground: Some(payload.new_height_above_ground), ..Default::default() }), ..Default::default() })
}
