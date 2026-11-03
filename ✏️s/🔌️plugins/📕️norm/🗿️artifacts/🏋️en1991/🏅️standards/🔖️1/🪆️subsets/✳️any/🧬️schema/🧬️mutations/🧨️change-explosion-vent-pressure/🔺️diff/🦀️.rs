//! 🧨 `change-explosion-vent-pressure` diff: patches `vent_pressure` of the addressed internal explosion row.

use crate::change_explosion_vent_pressure::ChangeExplosionVentPressure;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991InternalExplosionDelta, En1991InternalExplosionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeExplosionVentPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_vent_pressure.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.internal_explosions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Internal explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.vent_pressure == payload.new_vent_pressure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The internal explosion already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { internal_explosions: En1991InternalExplosionDelta::modification(&payload.id, En1991InternalExplosionPatch { vent_pressure: Some(payload.new_vent_pressure), ..Default::default() }), ..Default::default() })
}
