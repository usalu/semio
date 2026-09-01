//! 🔺️ `replace-step-measure` sparse diff construction — whole-value swap of an id-keyed
//! [`ProcessStep`]'s `measure` in the durable `step_payloads` timeline and re-mints
//! `steps`/`tool_solids` via
//! [`process3d_step_timeline_diff`](crate::artifacts::process3d::process3d_step_timeline_diff) —
//! `Cut`/`Attach` mint a tool solid, `Drill` mints none, exactly matching
//! `process_working_scene_to_snapshot`'s own per-measure minting. Error `target-missing` when the
//! step is absent, Warning `no-op` when the measure is unchanged.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{process3d_step_timeline_diff, Process3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceStepMeasure, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let Some(existing) = base.step_payloads.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.measure == payload.new_measure {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" measure is unchanged.", payload.id));
    }
    let mut steps = base.step_payloads.clone();
    if let Some(step) = steps.iter_mut().find(|step| step.id == payload.id) {
        step.measure = payload.new_measure.clone();
    }
    protocol::MutationOutcome::new(process3d_step_timeline_diff(base, steps))
}
//#endregion 🔖️Diff
