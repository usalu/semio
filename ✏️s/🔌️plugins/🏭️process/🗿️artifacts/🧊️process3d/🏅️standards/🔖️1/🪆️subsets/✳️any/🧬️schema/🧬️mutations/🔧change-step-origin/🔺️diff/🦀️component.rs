//! 🔺️ `change-step-origin` sparse diff construction — a single `Process3dStepsDelta.patched`
//! entry touching only `origin`, never a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepPatchEntry, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::change_step_origin::mutation::ChangeStepOrigin;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStepOrigin, _base: &Process3dSnapshot) -> Process3dDiff {
    let patch = ProcessStepPatch { origin: Some(payload.new_origin.clone()), ..Default::default() };
    Process3dDiff {
        steps: Some(Process3dStepsDelta { patched: vec![Process3dStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
