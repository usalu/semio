//! 🎯 CAD mutation — `SetActiveModelDefinition` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🎯 `SetActiveModelDefinition` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveModelDefinition {
    pub model_definition_id: String,
}

pub fn set_active_model_definition(model_definition_id: String) -> CadMutation {
    CadMutation::SetActiveModelDefinition { model_definition_id }
}

pub fn apply(projection: &mut CadSnapshot, model_definition_id: &str) {
    let mutation = CadMutation::SetActiveModelDefinition { model_definition_id: model_definition_id.into() };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
