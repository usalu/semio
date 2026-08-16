//! 🔺️ Sparse diff construction for the `disconnect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧹clear-adjacency` per Wave C.

use super::mutation::DisconnectAdjacency;
use crate::artifacts::program::diff::ProgramAdjacenciesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✂️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DisconnectAdjacency, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.adjacencies.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No adjacency exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
