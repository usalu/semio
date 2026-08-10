//! 🩹 CAD mutation — `PatchReference` payload + builder + apply.
use crate::artifacts::cad::mutations::{CadMutation, CadReferencePatch};
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🩹 `PatchReference` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchReference {
    pub model_definition_id: String,
    pub reference_id: String,
    pub patch: CadReferencePatch,
}

pub fn patch_reference(model_definition_id: String, reference_id: String, patch: CadReferencePatch) -> CadMutation {
    CadMutation::PatchReference { model_definition_id, reference_id, patch }
}

pub fn apply(projection: &mut CadSnapshot, model_definition_id: &str, reference_id: &str, patch: &CadReferencePatch) {
    let mutation = CadMutation::PatchReference { model_definition_id: model_definition_id.into(), reference_id: reference_id.into(), patch: patch.clone() };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
