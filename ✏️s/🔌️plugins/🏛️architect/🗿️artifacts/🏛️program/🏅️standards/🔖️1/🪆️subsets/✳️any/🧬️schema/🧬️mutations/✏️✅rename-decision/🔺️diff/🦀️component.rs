//! 🔺️ Sparse diff construction for the `rename-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::RenameDecision;
use crate::artifacts::program::diff::{ProgramDecisionsDelta, ProgramDecisionsPatchEntry};
use crate::artifacts::program::registers::DecisionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = DecisionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
