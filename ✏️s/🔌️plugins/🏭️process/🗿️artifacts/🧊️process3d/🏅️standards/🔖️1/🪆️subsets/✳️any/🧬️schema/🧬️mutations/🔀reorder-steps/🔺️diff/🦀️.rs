//! 🔺️ `reorder-steps` sparse diff construction — repositions an id-keyed [`ProcessStep`] within the durable `step_payloads` timeline,
//! matching `📥️insert-array-element`/`🔀reorder-columns`'s own remove-then-clamped-insert shape.
//! Error `target-missing` when the step is absent, Warning `no-op` when already at that position.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReorderSteps, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(from) = base.step_payloads.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if from == payload.to_index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" is already at position #{}.", payload.id, payload.to_index));
    }
    let to = payload.to_index.min(base.step_payloads.len() - 1);
    if to == from {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" is already at position #{}.", payload.id, to));
    }
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::relocation(&base.step_payloads, from, to)), ..Default::default() })
}
//#endregion 🔖️Diff
