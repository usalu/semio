//! ↩️ Inverse for `SetSnapshot`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔖️Inverse
