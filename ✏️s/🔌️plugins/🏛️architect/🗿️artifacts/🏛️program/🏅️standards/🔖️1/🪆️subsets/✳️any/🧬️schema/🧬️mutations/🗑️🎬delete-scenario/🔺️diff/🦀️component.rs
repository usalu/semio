//! 🔺️ Sparse diff construction for the `delete-scenario` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎬scenarios` per Wave C.

use super::mutation::DeleteScenario;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramScenariosDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteScenario, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { scenarios: Some(ProgramScenariosDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
