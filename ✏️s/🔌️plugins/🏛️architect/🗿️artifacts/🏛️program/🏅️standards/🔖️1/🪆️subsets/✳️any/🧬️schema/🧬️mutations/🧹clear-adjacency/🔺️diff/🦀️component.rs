//! 🔺️ Sparse diff construction for `DisconnectAdjacency`.

use super::mutation::DisconnectAdjacency;
use crate::artifacts::program::diff::ProgramAdjacenciesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✂️ `removed = [id]`.
pub fn diff_disconnect(payload: &DisconnectAdjacency, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { adjacencies: Some(ProgramAdjacenciesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
