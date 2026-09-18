//! 🔺️ Sparse diff builder for `DisconnectSlots` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::DisconnectSlots, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if !base.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc2dDiff { edges_removed: vec![payload.id.clone()], ..Default::default() })
}
