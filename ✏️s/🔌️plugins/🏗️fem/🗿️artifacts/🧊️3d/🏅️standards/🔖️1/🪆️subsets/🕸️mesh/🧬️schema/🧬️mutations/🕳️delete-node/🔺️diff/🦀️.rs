//! 🔺️ Sparse diff builder for `DeleteNode`.
//!
//! 🕳️ THE ONE VERB THAT DOES NOT REFUSE A REFERENCED TARGET. Every other `delete-` in this
//! vocabulary raises `mutation.target-referenced` while a referrer is alive; `delete-node` keeps
//! the permissive behaviour its own committed vector states in so many words —
//! `🧪️tests/🚫️removes-the-column-head-056295` asserts that frame `f1` keeps naming `n3` after the
//! node is gone ("delete-node is cascade-free"). Changing it would overturn a specified behaviour,
//! so the asymmetry is recorded here rather than silently removed. `node_referrers` in
//! `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` is the scan a future cascade would use.
use super::DeleteNode;
use crate::diff::{Fem3dDiff, Fem3dNodesDelta};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
