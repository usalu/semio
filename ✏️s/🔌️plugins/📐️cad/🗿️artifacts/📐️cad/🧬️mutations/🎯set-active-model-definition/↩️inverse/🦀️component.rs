//! ↩️ Inverse for `SetActiveModelDefinition`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;

//#region 🔖️Inverse
pub fn inverse(base: &CadProjection, _model_definition_id: &str) -> Vec<CadMutation> {
    vec![CadMutation::SetActiveModelDefinition { model_definition_id: base.active_model_definition_id.clone() }]
}
//#endregion 🔖️Inverse
