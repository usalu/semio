//! 🔺️ Sparse diff construction for the `disconnect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧹clear-adjacency` per Wave C.

use super::DisconnectAdjacency;
use crate::diff::ProgramAdjacenciesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✂️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DisconnectAdjacency, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.adjacencies.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No adjacency exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta::removal(&base.adjacencies, position)), ..Default::default() })
}
