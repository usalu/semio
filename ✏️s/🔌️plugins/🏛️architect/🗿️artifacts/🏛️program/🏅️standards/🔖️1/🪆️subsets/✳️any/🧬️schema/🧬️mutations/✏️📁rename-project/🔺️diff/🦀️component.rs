//! 🔺️ Sparse diff construction for the `rename-project` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📁update-project` per Wave C.

use super::mutation::RenameProject;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProjectDefinition` with only `code` changed.
pub fn diff(payload: &RenameProject, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.project.clone();
    value.code = payload.new_code.clone();
    ProgramDiff { project: Some(value), ..Default::default() }
}
