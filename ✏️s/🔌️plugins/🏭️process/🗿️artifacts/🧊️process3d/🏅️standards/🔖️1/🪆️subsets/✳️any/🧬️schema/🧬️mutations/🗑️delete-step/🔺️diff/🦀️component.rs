//! 🔺️ `delete-step` sparse diff construction — a single `Process3dStepsDelta.removed` entry,
//! never a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::delete_step::mutation::DeleteStep;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteStep, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { steps: Some(Process3dStepsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
