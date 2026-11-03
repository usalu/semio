//! ⚙ `change-local-fire-heat-release-rate` diff: patches `heat_release_rate` of the addressed localised fire row.

use crate::change_local_fire_heat_release_rate::ChangeLocalFireHeatReleaseRate;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LocalFireDelta, En1991LocalFirePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLocalFireHeatReleaseRate, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_heat_release_rate.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.local_fires.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Localised fire {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.heat_release_rate == payload.new_heat_release_rate {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The localised fire already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { local_fires: En1991LocalFireDelta::modification(&payload.id, En1991LocalFirePatch { heat_release_rate: Some(payload.new_heat_release_rate), ..Default::default() }), ..Default::default() })
}
