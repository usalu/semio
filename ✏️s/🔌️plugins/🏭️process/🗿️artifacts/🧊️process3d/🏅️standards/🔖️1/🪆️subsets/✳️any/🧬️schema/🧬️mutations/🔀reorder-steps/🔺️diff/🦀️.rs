//! 🔺️ `reorder-steps` sparse diff construction — repositions an id-keyed [`ProcessStep`] within
//! the durable `step_payloads` timeline and re-mints `steps`/`tool_solids` via
//! [`process3d_step_timeline_diff`](crate::artifacts::process3d::process3d_step_timeline_diff),
//! matching `📥️insert-array-element`/`🔀reorder-columns`'s own remove-then-clamped-insert shape.
//! Error `target-missing` when the step is absent, Warning `no-op` when already at that position.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReorderSteps, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(from) = base.step_payloads.iter().position(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if from == payload.to_index {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already at position #{}.", payload.id, payload.to_index));
    }
    let mut steps = base.step_payloads.clone();
    let step = steps.remove(from);
    let to = payload.to_index.min(steps.len());
    steps.insert(to, step);
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, steps))
}
//#endregion 🔖️Diff
