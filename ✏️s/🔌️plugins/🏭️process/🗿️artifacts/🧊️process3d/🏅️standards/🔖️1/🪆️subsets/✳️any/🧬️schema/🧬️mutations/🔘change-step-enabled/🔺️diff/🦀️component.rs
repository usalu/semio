//! 🔺️ `change-step-enabled` sparse diff construction — a single `Process3dStepsDelta.patched`
//! entry touching only `enabled`, never a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepPatchEntry, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::change_step_enabled::mutation::ChangeStepEnabled;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStepEnabled, _base: &Process3dSnapshot) -> Process3dDiff {
    let patch = ProcessStepPatch { enabled: Some(payload.new_enabled), ..Default::default() };
    Process3dDiff {
        steps: Some(Process3dStepsDelta { patched: vec![Process3dStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
