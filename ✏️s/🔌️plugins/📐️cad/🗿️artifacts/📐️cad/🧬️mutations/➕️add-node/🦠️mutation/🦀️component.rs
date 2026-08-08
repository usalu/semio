//! ➕️ CAD mutation — `AddNode` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadNode, CadProjection};
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

pub fn apply(projection: &mut CadProjection, node: &CadNode) {
    let mutation = CadMutation::AddNode { node: node.clone() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
