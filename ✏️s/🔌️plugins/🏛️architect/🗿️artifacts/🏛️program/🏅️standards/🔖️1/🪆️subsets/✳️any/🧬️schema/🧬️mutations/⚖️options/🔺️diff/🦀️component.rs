//! 🔺️ Sparse diff construction for the `options` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateOptionEvaluation, DeleteOptionEvaluation, RenameOptionEvaluation, ReplaceOptionEvaluation};
use crate::artifacts::program::diff::{ProgramOptionsDelta, ProgramOptionsPatchEntry};
use crate::artifacts::program::registers::OptionEvaluationPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.options` on apply.
pub fn diff_create(payload: &CreateOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { options: Some(ProgramOptionsDelta { added: vec![payload.option_evaluation.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { options: Some(ProgramOptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameOptionEvaluation, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OptionEvaluationPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { options: Some(ProgramOptionsDelta { patched: vec![ProgramOptionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceOptionEvaluation, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.option_evaluation).expect("diff_patch always produces a full patch");
    ProgramDiff { options: Some(ProgramOptionsDelta { patched: vec![ProgramOptionsPatchEntry { id: payload.option_evaluation.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
