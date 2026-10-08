//! 🔺️ `create-step` sparse diff construction — inserts a new [`ProcessStep`] into the durable
//! `step_payloads` timeline at `payload.index` (clamped to the timeline length) and re-mints
//! `steps`/`tool_solids` from the edited timeline via
//! [`process3d_step_timeline_diff`](crate::process3d_step_timeline_diff),
//! reusing `process_working_scene_to_snapshot`'s minting rather than duplicating it. Fatal
//! `duplicate-id` on an existing step id.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.step_payloads.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let at = payload.index.min(base.step_payloads.len());
    let reordered = (at < base.step_payloads.len()).then(|| base.step_payloads[..at].iter().map(|step| step.id.clone()).chain([payload.step.id.clone()]).chain(base.step_payloads[at..].iter().map(|step| step.id.clone())).collect());
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, Process3dStepsDelta { added: vec![payload.step.clone()], reordered, ..Default::default() }))
}
//#endregion 🔖️Diff
