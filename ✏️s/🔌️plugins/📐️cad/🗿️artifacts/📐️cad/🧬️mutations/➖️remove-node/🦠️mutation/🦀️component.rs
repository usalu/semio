//! ➖️ CAD mutation — `RemoveNode` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ➖️ `RemoveNode` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNode {
    pub node_id: String,
}

pub fn remove_node(node_id: String) -> CadMutation {
    CadMutation::RemoveNode { node_id }
}

pub fn apply(projection: &mut CadSnapshot, node_id: &str) {
    let mutation = CadMutation::RemoveNode { node_id: node_id.into() };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
