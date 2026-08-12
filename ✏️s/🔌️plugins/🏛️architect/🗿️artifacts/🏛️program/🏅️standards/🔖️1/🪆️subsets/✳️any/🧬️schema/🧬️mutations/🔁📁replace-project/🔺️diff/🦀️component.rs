//! 🔺️ Sparse diff construction for the `replace-project` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📁update-project` per Wave C.

use super::mutation::ReplaceProject;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `ProjectDefinition` wholesale.
pub fn diff(payload: &ReplaceProject, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { project: Some(payload.new_project.clone()), ..Default::default() }
}
