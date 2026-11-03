//! ⚙ `remove-rotating-machine` diff: removes the addressed rotating machine row.

use crate::remove_rotating_machine::RemoveRotatingMachine;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RotatingMachineDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRotatingMachine, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.rotating_machines.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rotating machine {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { rotating_machines: En1991RotatingMachineDelta::removal(&base.rotating_machines, index), ..Default::default() })
}
