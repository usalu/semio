//! 🔺️ Sparse diff builder for `CreateNode`.
use super::CreateNode;
use semio_s_artifact_cad_cad_model::{diff::CadDiff, diff::CadNodesDelta};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    let index = payload.index.map_or(base.nodes.len(), |index| index as usize);
    if index > base.nodes.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node insertion index {index} exceeds the {} existing nodes.", base.nodes.len()), [payload.node.id.clone()]);
    }
    protocol::MutationOutcome::new(CadDiff { nodes: Some(CadNodesDelta::insertion(index, payload.node.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
