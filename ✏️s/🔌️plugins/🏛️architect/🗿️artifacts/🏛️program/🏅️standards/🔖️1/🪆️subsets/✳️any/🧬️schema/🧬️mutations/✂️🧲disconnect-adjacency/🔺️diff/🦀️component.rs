//! 🔺️ Sparse diff construction for the `disconnect-adjacency` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧹clear-adjacency` per Wave C.

use super::mutation::DisconnectAdjacency;
use crate::artifacts::program::diff::ProgramAdjacenciesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✂️ `removed = [id]`.
pub fn diff(payload: &DisconnectAdjacency, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
