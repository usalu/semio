//! 🔺️ Sparse diff construction for the `delete-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::DeleteRisk;
use crate::artifacts::program::diff::ProgramRisksDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { risks: Some(ProgramRisksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
