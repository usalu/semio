//! 🔺️ Sparse diff construction for the `replace-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::mutation::ReplaceScenario;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramScenariosDelta, ProgramScenariosPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceScenario, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.scenarios.iter().find(|row| row.header.id == payload.scenario.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.scenario).expect("diff_patch always produces a full patch");
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.scenario.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
