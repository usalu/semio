//! 🔺️ `replace-step-measure` sparse diff construction — whole-value swap of an id-keyed [`ProcessStep`]'s `measure` in the durable
//! `step_payloads` timeline; `apply` re-derives the `steps`/`tool_solids` handles (`Cut`/`Attach` mint a tool solid, `Drill` none).
//! Error `target-missing` when the step is absent, Warning `no-op` when the measure is unchanged.

use crate::diff::{Process3dOptionalOrigin, Process3dStepPatch, Process3dStepsDelta};
use crate::diff::Process3dDiff;
use crate::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceStepMeasure, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.step_payloads.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.measure == payload.new_measure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" measure is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(Process3dDiff { step_payloads: Some(Process3dStepsDelta::modification(payload.id.clone(), Process3dStepPatch { measure: Some(payload.new_measure.clone()), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
