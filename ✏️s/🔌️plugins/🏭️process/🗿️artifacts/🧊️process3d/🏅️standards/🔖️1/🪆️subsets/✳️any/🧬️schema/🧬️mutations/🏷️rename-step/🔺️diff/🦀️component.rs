//! 🔺️ `rename-step` sparse diff construction — a single `Process3dStepsDelta.patched` entry
//! touching only `label`, never a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepPatchEntry, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::rename_step::mutation::RenameStep;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStepPatch};

//#region 🔖️Diff
pub fn diff(payload: &RenameStep, _base: &Process3dSnapshot) -> Process3dDiff {
    let patch = ProcessStepPatch { label: Some(payload.new_label.clone()), ..Default::default() };
    Process3dDiff {
        steps: Some(Process3dStepsDelta { patched: vec![Process3dStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
