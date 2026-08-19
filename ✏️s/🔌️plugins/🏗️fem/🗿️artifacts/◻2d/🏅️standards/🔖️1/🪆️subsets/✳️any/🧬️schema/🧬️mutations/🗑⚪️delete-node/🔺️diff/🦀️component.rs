//! 🔺️ Sparse diff builder for `DeleteNode`.
use super::mutation::DeleteNode;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteNode, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
