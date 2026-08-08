//! 🏷️ CAD mutation — `RenameNode` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🏷️ `RenameNode` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameNode {
    pub node_id: String,
    pub label: String,
}

pub fn rename_node(node_id: String, label: String) -> CadMutation {
    CadMutation::RenameNode { node_id, label }
}

pub fn apply(projection: &mut CadProjection, node_id: &str, label: &str) {
    let mutation = CadMutation::RenameNode { node_id: node_id.into(), label: label.into() };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
