//! ➕️ CAD mutation — `AddObject` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadProjection};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ➕️ `AddObject` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddObject {
    pub pane: CadPaneId,
    pub object: CadObject,
}

pub fn add_object(pane: CadPaneId, object: CadObject) -> CadMutation {
    CadMutation::AddObject { pane, object }
}

pub fn apply(projection: &mut CadProjection, pane: CadPaneId, object: &CadObject) {
    let mutation = CadMutation::AddObject { pane, object: object.clone() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
