//! 🔺️ Sparse diff builder for `DisconnectSlots` — removes the id from `edges`.

use crate::diff::{Wfc3dDiff, Wfc3dEdgesDelta};
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DisconnectSlots, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { edges: Wfc3dEdgesDelta::removal(&base.edges, base.edges.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), ..Default::default() })
}
