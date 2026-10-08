//! 🔺️ `rename-step` sparse diff construction — sets an id-keyed [`ProcessStep`]'s `label` in the durable `step_payloads` timeline.
//! Error `target-missing` when the step is absent, Warning `no-op` when the new label equals the
//! old (step `label` is a display string, not a key, so no `duplicate-id` case applies here). The derived `steps`/`tool_solids`
//! handles are re-derived by `apply`, never carried.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameStep, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.step_payloads.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" is already named \"{}\".", payload.id, payload.new_label));
    }
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::modification(payload.id.clone(), Process3dStepPatch { label: Some(payload.new_label.clone()), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
