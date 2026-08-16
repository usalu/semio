//! 🔺️ Sparse diff construction for the `create-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::CreateDecision;
use crate::artifacts::program::diff::ProgramDecisionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.decisions` on apply.
pub fn diff(payload: &CreateDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { added: vec![payload.decision.clone()], ..Default::default() }), ..Default::default() }
}
