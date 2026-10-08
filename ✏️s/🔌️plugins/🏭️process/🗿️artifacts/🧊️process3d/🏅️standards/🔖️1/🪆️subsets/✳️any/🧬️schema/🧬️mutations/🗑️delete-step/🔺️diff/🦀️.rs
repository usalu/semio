//! 🔺️ `delete-step` sparse diff construction — removes an id-keyed [`ProcessStep`] from the durable `step_payloads` timeline;
//! `apply` re-derives the `steps`/`tool_solids` handles. Error `target-missing` when the step is absent.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(index) = base.step_payloads.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::removal(&base.step_payloads, index)), ..Default::default() })
}
//#endregion 🔖️Diff
