//! 🔺️ Sparse diff builder for `RenameNode`.
use super::RenameNode;
use crate::artifacts::cad::diff::{CadDiff, CadNodePatchEntry, CadNodesDelta};
use crate::artifacts::cad::mutations::CadNodePatch;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already named \"{}\".", payload.node_id, payload.new_label));
    }
    protocol::MutationOutcome::new(CadDiff {
        nodes: Some(CadNodesDelta { patched: vec![CadNodePatchEntry { id: payload.node_id.clone(), patch: CadNodePatch { label: Some(payload.new_label.clone()) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
