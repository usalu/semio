//! 🔺️ `create-step` sparse diff construction — inserts a new [`ProcessStep`] into the durable
//! `step_payloads` timeline at `payload.index` (clamped to the timeline length) and re-mints
//! `steps`/`tool_solids` from the edited timeline via
//! [`process3d_step_timeline_diff`](crate::artifacts::process3d::process3d_step_timeline_diff),
//! reusing `process_working_scene_to_snapshot`'s minting rather than duplicating it. Fatal
//! `duplicate-id` on an existing step id.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.step_payloads.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let mut steps = base.step_payloads.clone();
    let index = payload.index.min(steps.len());
    steps.insert(index, payload.step.clone());
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, steps))
}
//#endregion 🔖️Diff
