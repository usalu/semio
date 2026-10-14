//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::DeleteNode;
use semio_s_artifact_cad_cad_model::{diff::CadDiff, diff::CadNodesDelta};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(index) = base.nodes.iter().position(|node| node.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    protocol::MutationOutcome::new(CadDiff { nodes: Some(CadNodesDelta::removal(&base.nodes, index)), ..Default::default() })
}
//#endregion 🔖️Diff
