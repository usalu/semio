//! 🩹 CAD mutation — `PatchObject` payload + builder + apply.
use crate::artifacts::cad::mutations::{CadMutation, CadObjectPatch};
use crate::artifacts::cad::{CadPaneId, CadProjection};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🩹 `PatchObject` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchObject {
    pub pane: CadPaneId,
    pub object_id: String,
    pub patch: CadObjectPatch,
}

pub fn patch_object(pane: CadPaneId, object_id: String, patch: CadObjectPatch) -> CadMutation {
    CadMutation::PatchObject { pane, object_id, patch }
}

pub fn apply(projection: &mut CadProjection, pane: CadPaneId, object_id: &str, patch: &CadObjectPatch) {
    let mutation = CadMutation::PatchObject { pane, object_id: object_id.into(), patch: patch.clone() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
