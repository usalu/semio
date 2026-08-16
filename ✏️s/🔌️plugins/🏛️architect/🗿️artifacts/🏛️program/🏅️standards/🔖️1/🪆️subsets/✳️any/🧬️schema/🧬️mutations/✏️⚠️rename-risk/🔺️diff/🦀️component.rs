//! 🔺️ Sparse diff construction for the `rename-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::RenameRisk;
use crate::artifacts::program::diff::{ProgramRisksDelta, ProgramRisksPatchEntry};
use crate::artifacts::program::registers::RiskPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RiskPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
