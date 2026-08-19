//! 🔺️ Sparse diff builder for `CreateNode`.
use super::mutation::CreateNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodesDelta};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    protocol::MutationOutcome::new(CadDiff { nodes: Some(CadNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
