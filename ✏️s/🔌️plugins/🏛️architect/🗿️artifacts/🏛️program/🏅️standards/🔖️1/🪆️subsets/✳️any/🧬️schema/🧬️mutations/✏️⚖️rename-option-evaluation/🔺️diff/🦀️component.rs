//! 🔺️ Sparse diff construction for the `rename-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::mutation::RenameOptionEvaluation;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramOptionsDelta, ProgramOptionsPatchEntry};
use crate::artifacts::program::registers::OptionEvaluationPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OptionEvaluationPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { options: Some(ProgramOptionsDelta { patched: vec![ProgramOptionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
