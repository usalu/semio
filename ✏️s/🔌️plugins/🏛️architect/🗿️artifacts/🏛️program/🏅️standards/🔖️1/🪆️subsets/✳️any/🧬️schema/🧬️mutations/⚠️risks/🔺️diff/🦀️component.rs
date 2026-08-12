//! 🔺️ Sparse diff construction for the `risks` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateRisk, DeleteRisk, RenameRisk, ReplaceRisk};
use crate::artifacts::program::diff::{ProgramRisksDelta, ProgramRisksPatchEntry};
use crate::artifacts::program::registers::RiskPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.risks` on apply.
pub fn diff_create(payload: &CreateRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { risks: Some(ProgramRisksDelta { added: vec![payload.risk.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { risks: Some(ProgramRisksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameRisk, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RiskPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceRisk, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.risks.iter().find(|row| row.header.id == payload.risk.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.risk).expect("diff_patch always produces a full patch");
    ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.risk.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
