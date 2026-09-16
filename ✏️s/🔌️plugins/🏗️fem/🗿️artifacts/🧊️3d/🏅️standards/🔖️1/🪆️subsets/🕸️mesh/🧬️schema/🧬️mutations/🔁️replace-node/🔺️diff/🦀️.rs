//! 🔺️ Sparse diff builder for `ReplaceNode`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME finite-position bound
//! `create-node` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceNode;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dNodesDelta, Fem3dNodesPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{id_mismatch, invariant, node_breach};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_node.id != payload.id {
        return id_mismatch("Node", &payload.id, &payload.new_node.id);
    }
    if let Some(breach) = node_breach(&payload.new_node) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    if *existing == payload.new_node {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { patched: vec![Fem3dNodesPatchEntry { id: payload.id.clone(), item: payload.new_node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
