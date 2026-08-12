//! ↩️ Inverse (undo) construction for the `update_project` mutation leaf.

use super::mutation::{RenameProject, ReplaceProject};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub fn inverse_rename(_payload: &RenameProject, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameProject(RenameProject { new_code: base.project.code.clone() })]
}

pub fn inverse_replace(_payload: &ReplaceProject, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceProject(ReplaceProject { new_project: base.project.clone() })]
}
