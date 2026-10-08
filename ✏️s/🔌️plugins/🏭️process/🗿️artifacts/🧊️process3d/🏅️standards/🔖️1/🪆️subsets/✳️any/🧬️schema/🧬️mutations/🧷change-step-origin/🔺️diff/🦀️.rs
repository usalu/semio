//! 🔺️ `change-step-origin` sparse diff construction — sets (or clears) an id-keyed [`ProcessStep`]'s `origin` provenance in the
//! durable `step_payloads` timeline. Error `target-missing` when the step is absent, Warning `no-op` when the origin is unchanged.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeStepOrigin, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.step_payloads.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.origin == payload.new_origin {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" origin is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::modification(payload.id.clone(), Process3dStepPatch { origin: Some(Process3dOptionalOrigin { value: payload.new_origin.clone() }), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
