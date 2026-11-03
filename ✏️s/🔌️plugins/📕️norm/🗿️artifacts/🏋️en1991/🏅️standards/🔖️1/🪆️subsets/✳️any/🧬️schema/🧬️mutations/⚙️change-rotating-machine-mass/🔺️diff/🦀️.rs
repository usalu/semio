//! ⚙ `change-rotating-machine-mass` diff: patches `mass` of the addressed rotating machine row.

use crate::change_rotating_machine_mass::ChangeRotatingMachineMass;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RotatingMachineDelta, En1991RotatingMachinePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeRotatingMachineMass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_mass.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.rotating_machines.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rotating machine {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.mass == payload.new_mass {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The rotating machine already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { rotating_machines: En1991RotatingMachineDelta::modification(&payload.id, En1991RotatingMachinePatch { mass: Some(payload.new_mass), ..Default::default() }), ..Default::default() })
}
