//! ✏️ `change-bridge-period-ratio` diff: patches `period_ratio` of the addressed bridge row.

use crate::change_bridge_period_ratio::ChangeBridgePeriodRatio;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgePeriodRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_period_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.period_ratio == payload.new_period_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.id, En1998BridgePatch { period_ratio: Some(payload.new_period_ratio), ..Default::default() }), ..Default::default() })
}
