//! 📎 CAD mutation — `SetReferences` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadReference, CadSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 📎 `SetReferences` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetReferences {
    pub model_definition_id: String,
    pub references: Vec<CadReference>,
}

pub fn set_references(model_definition_id: String, references: Vec<CadReference>) -> CadMutation {
    CadMutation::SetReferences { model_definition_id, references }
}

pub fn apply(projection: &mut CadSnapshot, model_definition_id: &str, references: &[CadReference]) {
    let mutation = CadMutation::SetReferences { model_definition_id: model_definition_id.into(), references: references.to_vec() };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
