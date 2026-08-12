//! 🔺️ Sparse diff construction for the `create-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::mutation::CreateOptionEvaluation;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramOptionsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.options` on apply.
pub fn diff(payload: &CreateOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { options: Some(ProgramOptionsDelta { added: vec![payload.option_evaluation.clone()], ..Default::default() }), ..Default::default() }
}
