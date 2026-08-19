//! 🔺️ Sparse diff builder for `DisconnectSlots` — removes the id from `edges`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn diff(payload: &super::mutation::DisconnectSlots, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(AssemblyDiff { edges_removed: vec![payload.id.clone()], ..Default::default() })
}
