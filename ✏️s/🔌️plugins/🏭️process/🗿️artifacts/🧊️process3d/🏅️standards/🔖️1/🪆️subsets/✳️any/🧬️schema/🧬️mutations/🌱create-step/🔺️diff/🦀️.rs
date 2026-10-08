//! 🔺️ `create-step` sparse diff construction — inserts a new [`ProcessStep`] into the durable `step_payloads` timeline at
//! `payload.index` (clamped to the timeline length); `apply` re-derives the `steps`/`tool_solids` handles. Fatal `duplicate-id` on an
//! existing step id.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.step_payloads.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let at = payload.index.min(base.step_payloads.len());
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::insertion(at, payload.step.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
