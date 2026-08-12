//! 🔺️ Sparse diff construction for the `decisions` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateDecision, DeleteDecision, RenameDecision, ReplaceDecision};
use crate::artifacts::program::diff::{ProgramDecisionsDelta, ProgramDecisionsPatchEntry};
use crate::artifacts::program::registers::DecisionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.decisions` on apply.
pub fn diff_create(payload: &CreateDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { added: vec![payload.decision.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameDecision, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = DecisionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceDecision, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.decisions.iter().find(|row| row.header.id == payload.decision.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.decision).expect("diff_patch always produces a full patch");
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.decision.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
