//! ➕️ CAD mutation — `AddNode` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadNode, CadSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ➕️ `AddNode` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddNode {
    pub node: CadNode,
}

pub fn add_node(node: CadNode) -> CadMutation {
    CadMutation::AddNode { node }
}

pub fn apply(projection: &mut CadSnapshot, node: &CadNode) {
    let mutation = CadMutation::AddNode { node: node.clone() };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
