//! 🔺️ Sparse diff builder for `DeleteNode`.
//!
//! One guard only: `mutation.target-missing` (Error). `delete-node` is the ONE deliberately
//! cascade-free `delete-` in this vocabulary — a plan node is a drafting coordinate, and dropping
//! one while an element, a support or a nodal load still names it is the SPECIFIED behaviour, not
//! an oversight; the committed vector `🚫️removes-node-n3-without-6eab3f` pins it. Every other
//! guarded `delete-` refuses with `mutation.target-referenced` instead (see `mutations::guards`).
use super::DeleteNode;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
