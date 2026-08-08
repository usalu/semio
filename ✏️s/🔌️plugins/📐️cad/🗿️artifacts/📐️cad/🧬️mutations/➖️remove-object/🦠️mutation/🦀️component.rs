//! ➖️ CAD mutation — `RemoveObject` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadProjection};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ➖️ `RemoveObject` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveObject {
    pub pane: CadPaneId,
    pub object_id: String,
}

pub fn remove_object(pane: CadPaneId, object_id: String) -> CadMutation {
    CadMutation::RemoveObject { pane, object_id }
}

pub fn apply(projection: &mut CadProjection, pane: CadPaneId, object_id: &str) {
    let mutation = CadMutation::RemoveObject { pane, object_id: object_id.into() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
