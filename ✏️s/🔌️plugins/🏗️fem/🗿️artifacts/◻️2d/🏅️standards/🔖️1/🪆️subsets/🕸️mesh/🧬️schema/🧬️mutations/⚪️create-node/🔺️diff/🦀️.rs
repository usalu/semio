//! 🔺️ Sparse diff builder for `CreateNode`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_geometry` finiteness bound (`mutation.invariant`, Fatal).
use super::CreateNode;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    if let Some(rejection) = guards::node_geometry(&payload.node) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
