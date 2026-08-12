//! 🔺️ `reorder-steps` sparse diff construction — a single `Process3dStepsDelta.reordered` full
//! id-order list, built directly from `base`, never a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::reorder_steps::mutation::ReorderSteps;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReorderSteps, base: &Process3dSnapshot) -> Process3dDiff {
    let mut ids: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    Process3dDiff { steps: Some(Process3dStepsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
