//! 🔺️ Sparse diff construction for the `create-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::CreateRisk;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRisksDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.risks` on apply.
pub fn diff(payload: &CreateRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { risks: Some(ProgramRisksDelta { added: vec![payload.risk.clone()], ..Default::default() }), ..Default::default() }
}
