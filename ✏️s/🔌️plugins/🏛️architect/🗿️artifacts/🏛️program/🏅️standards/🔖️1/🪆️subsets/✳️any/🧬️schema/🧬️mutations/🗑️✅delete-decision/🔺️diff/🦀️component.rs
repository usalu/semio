//! 🔺️ Sparse diff construction for the `delete-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::DeleteDecision;
use crate::artifacts::program::diff::ProgramDecisionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
