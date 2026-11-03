//! 🛣️ `change-lane-lm1-lane` diff: patches `lm1_lane` of the addressed bridge lane row.

use crate::change_lane_lm1_lane::ChangeLaneLm1Lane;
use semio_s_artifact_norm_en1991_diff::{En1991BridgeLaneDelta, En1991BridgeLanePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLaneLm1Lane, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.bridge_lanes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge lane {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.lm1_lane == payload.new_lm1_lane {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge lane already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lanes: En1991BridgeLaneDelta::modification(&payload.id, En1991BridgeLanePatch { lm1_lane: Some(payload.new_lm1_lane), ..Default::default() }), ..Default::default() })
}
