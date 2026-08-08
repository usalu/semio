//! ↩️ Inverse for `SetScene`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;

//#region 🔖️Inverse
pub fn inverse(base: &CadProjection, _scene: &CadProjection) -> Vec<CadMutation> {
    vec![CadMutation::SetScene { scene: Box::new(base.clone()) }]
}
//#endregion 🔖️Inverse
