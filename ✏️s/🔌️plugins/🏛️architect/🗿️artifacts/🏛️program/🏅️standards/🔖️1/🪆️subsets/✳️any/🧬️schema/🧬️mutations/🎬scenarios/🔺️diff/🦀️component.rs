//! 🔺️ Sparse diff construction for the `scenarios` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateScenario, DeleteScenario, RenameScenario, ReplaceScenario};
use crate::artifacts::program::diff::{ProgramScenariosDelta, ProgramScenariosPatchEntry};
use crate::artifacts::program::registers::ScenarioPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.scenarios` on apply.
pub fn diff_create(payload: &CreateScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { added: vec![payload.scenario.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ScenarioPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceScenario, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.scenarios.iter().find(|row| row.header.id == payload.scenario.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.scenario).expect("diff_patch always produces a full patch");
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.scenario.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
