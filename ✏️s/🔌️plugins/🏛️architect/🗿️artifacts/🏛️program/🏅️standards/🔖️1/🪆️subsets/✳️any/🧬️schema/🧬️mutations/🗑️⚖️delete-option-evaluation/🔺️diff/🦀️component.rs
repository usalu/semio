//! 🔺️ Sparse diff construction for the `delete-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::mutation::DeleteOptionEvaluation;
use crate::artifacts::program::diff::ProgramOptionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { options: Some(ProgramOptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
