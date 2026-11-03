//! ⬇ `change-falling-mass` diff: patches `mass` of the addressed falling load row.

use crate::change_falling_mass::ChangeFallingMass;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FallingLoadDelta, En1991FallingLoadPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFallingMass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_mass.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.falling_loads.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Falling load {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.mass == payload.new_mass {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The falling load already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { falling_loads: En1991FallingLoadDelta::modification(&payload.id, En1991FallingLoadPatch { mass: Some(payload.new_mass), ..Default::default() }), ..Default::default() })
}
