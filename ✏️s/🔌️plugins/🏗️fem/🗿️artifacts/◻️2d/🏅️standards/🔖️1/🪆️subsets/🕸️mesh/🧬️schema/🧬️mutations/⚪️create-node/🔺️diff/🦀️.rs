//! 🔺️ Sparse diff builder for `CreateNode`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_geometry` finiteness bound (`mutation.invariant`, Fatal).
use super::CreateNode;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodeInsertion, Fem2dNodesDelta};
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
    if payload.index.is_some_and(|at| at > base.nodes.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.nodes.len()), [&payload.node.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { inserted: vec![Fem2dNodeInsertion { index: payload.index.unwrap_or(base.nodes.len()), row: payload.node.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
