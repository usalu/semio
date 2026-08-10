//! ↩️ Inverse for `SetCursor`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &Process3dSnapshot, _resolved_up_to: Option<usize>) -> Vec<Process3dMutation> {
    vec![Process3dMutation::SetCursor { resolved_up_to: base.resolved_up_to }]
}
//#endregion 🔖️Inverse
