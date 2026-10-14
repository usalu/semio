//! 🔺️ Sparse diff builder for `RenameNode`.
use super::RenameNode;
use semio_s_artifact_cad_cad_model::{diff::CadDiff, diff::CadNodesDelta};
use semio_s_artifact_cad_cad_model::diff::CadNodePatch;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already named \"{}\".", payload.node_id, payload.new_label));
    }
    protocol::MutationOutcome::new(CadDiff {
        nodes: Some(CadNodesDelta::modification(payload.node_id.clone(), CadNodePatch { label: Some(payload.new_label.clone()) })),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
