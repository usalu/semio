//! 🔺️ `rename-step` sparse diff construction — sets an id-keyed [`ProcessStep`]'s `label` in the
//! durable `step_payloads` timeline and re-mints `steps`/`tool_solids` via
//! [`process3d_step_timeline_diff`](crate::artifacts::process3d::process3d_step_timeline_diff).
//! Error `target-missing` when the step is absent, Warning `no-op` when the new label equals the
//! old (step `label` is a display string, not a key, so no `duplicate-id` case applies here).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::RenameStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.step_payloads.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already named \"{}\".", payload.id, payload.new_label));
    }
    let mut steps = base.step_payloads.clone();
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.id) {
        step.label = payload.new_label.clone();
    }
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, steps))
}
//#endregion 🔖️Diff
