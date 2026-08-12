//! 🔺️ Sparse diff construction for the `update_project` mutation leaf.

use super::mutation::{RenameProject, ReplaceProject};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProjectDefinition` with only `code` changed.
pub fn diff_rename(payload: &RenameProject, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.project.clone();
    value.code = payload.new_code.clone();
    ProgramDiff { project: Some(value), ..Default::default() }
}

/// 🔁️ New `ProjectDefinition` wholesale.
pub fn diff_replace(payload: &ReplaceProject, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { project: Some(payload.new_project.clone()), ..Default::default() }
}
