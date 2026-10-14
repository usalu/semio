//! 🔺️ `delete-machine` sparse diff construction — a whole-`Workshop` value diff, built directly
//! from `base` + payload, never a snapshot clone. Error `target-missing` when the machine is absent.

use semio_s_artifact_process_process3d_model_diff::{Process3dDiff, Process3dMachinesDelta};
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteMachine, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(index) = base.workshop.machines.iter().position(|machine| machine.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Machine \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Process3dDiff { workshop: Some(Process3dMachinesDelta::removal(&base.workshop.machines, index)), ..Default::default() })
}
//#endregion 🔖️Diff
