//! 🔺️ Sparse diff construction for the `rename-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::mutation::RenameScenario;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramScenariosDelta, ProgramScenariosPatchEntry};
use crate::artifacts::program::registers::ScenarioPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ScenarioPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { patched: vec![ProgramScenariosPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
