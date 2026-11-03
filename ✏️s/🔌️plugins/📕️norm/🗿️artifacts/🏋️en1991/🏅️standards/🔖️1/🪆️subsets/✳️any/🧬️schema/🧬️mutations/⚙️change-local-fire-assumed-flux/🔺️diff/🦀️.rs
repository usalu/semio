//! ⚙ `change-local-fire-assumed-flux` diff: patches `assumed_flux` of the addressed localised fire row.

use crate::change_local_fire_assumed_flux::ChangeLocalFireAssumedFlux;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LocalFireDelta, En1991LocalFirePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLocalFireAssumedFlux, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_flux.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.local_fires.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Localised fire {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_flux == payload.new_assumed_flux {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The localised fire already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { local_fires: En1991LocalFireDelta::modification(&payload.id, En1991LocalFirePatch { assumed_flux: Some(payload.new_assumed_flux), ..Default::default() }), ..Default::default() })
}
