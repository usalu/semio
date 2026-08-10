//! ↩️ Inverse for `SetSnapshot`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, _scene: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::SetSnapshot { snapshot: Box::new(base.clone()) }]
}
//#endregion 🔖️Inverse
