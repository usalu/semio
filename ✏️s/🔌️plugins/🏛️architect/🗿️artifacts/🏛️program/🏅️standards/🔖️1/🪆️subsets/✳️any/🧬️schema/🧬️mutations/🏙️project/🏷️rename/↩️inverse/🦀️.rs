//! ↩️ Inverse (undo) construction for the `rename-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub async fn inverse(_payload: &super::RenameProject, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameProject(super::RenameProject { new_code: base.project.code.clone() })]
}
