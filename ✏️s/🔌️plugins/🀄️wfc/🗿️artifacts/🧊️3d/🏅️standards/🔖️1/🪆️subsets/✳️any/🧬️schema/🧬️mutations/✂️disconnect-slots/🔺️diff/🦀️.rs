//! 🔺️ Sparse diff builder for `DisconnectSlots` — removes the id from `edges`.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::DisconnectSlots, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if !base.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("wfc3d.edge.missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Wfc3dDiff { edges_removed: vec![payload.id.clone()], ..Default::default() })
}
