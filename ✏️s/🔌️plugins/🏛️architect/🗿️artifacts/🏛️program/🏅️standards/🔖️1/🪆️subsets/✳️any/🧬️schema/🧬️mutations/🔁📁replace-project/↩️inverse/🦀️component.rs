//! ↩️ Inverse (undo) construction for the `replace-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub fn inverse(_payload: &super::mutation::ReplaceProject, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceProject(super::mutation::ReplaceProject { new_project: base.project.clone() })]
}
