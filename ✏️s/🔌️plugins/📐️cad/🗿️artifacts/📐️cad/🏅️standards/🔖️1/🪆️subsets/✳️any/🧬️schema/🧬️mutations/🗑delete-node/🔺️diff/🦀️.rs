//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::DeleteNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodesDelta};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if !base.nodes.iter().any(|node| node.id == payload.node_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    }
    protocol::MutationOutcome::new(CadDiff { nodes: Some(CadNodesDelta { removed: vec![payload.node_id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
