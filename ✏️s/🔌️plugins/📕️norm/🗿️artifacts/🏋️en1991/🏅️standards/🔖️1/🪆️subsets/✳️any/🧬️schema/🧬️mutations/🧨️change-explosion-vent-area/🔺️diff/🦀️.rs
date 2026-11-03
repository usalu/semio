//! 🧨 `change-explosion-vent-area` diff: patches `vent_area` of the addressed internal explosion row.

use crate::change_explosion_vent_area::ChangeExplosionVentArea;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991InternalExplosionDelta, En1991InternalExplosionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeExplosionVentArea, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_vent_area.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.internal_explosions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Internal explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.vent_area == payload.new_vent_area {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The internal explosion already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { internal_explosions: En1991InternalExplosionDelta::modification(&payload.id, En1991InternalExplosionPatch { vent_area: Some(payload.new_vent_area), ..Default::default() }), ..Default::default() })
}
