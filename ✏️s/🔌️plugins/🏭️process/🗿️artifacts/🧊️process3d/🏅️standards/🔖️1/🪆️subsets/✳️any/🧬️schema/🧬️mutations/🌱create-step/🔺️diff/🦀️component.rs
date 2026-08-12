//! 🔺️ `create-step` sparse diff construction — a single `Process3dStepsDelta.added` entry, never
//! a snapshot clone.

use crate::artifacts::process3d::diff::{Process3dDiff, Process3dStepsDelta};
use crate::artifacts::process3d::mutations::create_step::mutation::CreateStep;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
/// 🏗️ Builds the sparse steps delta for one `create-step` payload.
pub fn diff(payload: &CreateStep, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { steps: Some(Process3dStepsDelta { added: vec![payload.step.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
