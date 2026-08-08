//! 🖼️ CAD mutation — `SetPaneObjects` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadProjection};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🖼️ `SetPaneObjects` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPaneObjects {
    pub pane: CadPaneId,
    pub objects: Vec<CadObject>,
}

pub fn set_pane_objects(pane: CadPaneId, objects: Vec<CadObject>) -> CadMutation {
    CadMutation::SetPaneObjects { pane, objects }
}

pub fn apply(projection: &mut CadProjection, pane: CadPaneId, objects: &[CadObject]) {
    let mutation = CadMutation::SetPaneObjects { pane, objects: objects.to_vec() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
