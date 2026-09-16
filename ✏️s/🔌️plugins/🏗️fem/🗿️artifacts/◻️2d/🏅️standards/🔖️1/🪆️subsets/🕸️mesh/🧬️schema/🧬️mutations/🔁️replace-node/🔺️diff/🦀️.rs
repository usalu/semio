//! 🔺️ Sparse diff builder for `ReplaceNode`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME `node_geometry` bound
//! `create-node` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceNode;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodesDelta, Fem2dNodesPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceNode, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("node", &payload.id, &payload.new_node.id) {
        return rejection;
    }
    if let Some(rejection) = guards::node_geometry(&payload.new_node) {
        return rejection;
    }
    if *existing == payload.new_node {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { patched: vec![Fem2dNodesPatchEntry { id: payload.id.clone(), item: payload.new_node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
