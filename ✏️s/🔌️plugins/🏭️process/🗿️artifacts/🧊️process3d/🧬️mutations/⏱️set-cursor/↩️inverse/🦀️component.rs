//! ↩️ Inverse for `SetCursor`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dDocument;

//#region 🔖️Inverse
pub fn inverse(base: &Process3dDocument, _resolved_up_to: Option<usize>) -> Vec<Process3dMutation> {
    vec![Process3dMutation::SetCursor { resolved_up_to: base.resolved_up_to }]
}
//#endregion 🔖️Inverse
