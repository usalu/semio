//! 🔺️ `delete-step` sparse diff construction — removes an id-keyed [`ProcessStep`] from the
//! durable `step_payloads` timeline and re-mints `steps`/`tool_solids` from the edited timeline via
//! [`process3d_step_timeline_diff`](crate::artifacts::process3d::process3d_step_timeline_diff).
//! Error `target-missing` when the step is absent.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if !base.step_payloads.iter().any(|step| step.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut steps = base.step_payloads.clone();
    steps.retain(|step| step.id != payload.id);
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, steps))
}
//#endregion 🔖️Diff
