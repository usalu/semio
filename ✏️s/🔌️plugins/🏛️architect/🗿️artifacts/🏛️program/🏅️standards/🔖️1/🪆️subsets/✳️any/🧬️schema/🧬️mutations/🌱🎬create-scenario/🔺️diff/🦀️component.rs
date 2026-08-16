//! 🔺️ Sparse diff construction for the `create-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::mutation::CreateScenario;
use crate::artifacts::program::diff::ProgramScenariosDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.scenarios` on apply.
pub fn diff(payload: &CreateScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { added: vec![payload.scenario.clone()], ..Default::default() }), ..Default::default() }
}
