//! ↩️ Inverse for `ChangeActiveModelDefinition` — recovers the pre-mutation selector from `base`.
use super::ChangeActiveModelDefinition;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeActiveModelDefinition, base: &CadSnapshot) -> Vec<CadMutation> {
    vec![CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: base.active_model_definition_id.clone() })]
}
//#endregion 🔖️Inverse
