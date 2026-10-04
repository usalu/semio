//! ↩️ Inverse (undo) construction for the `replace-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

pub fn inverse(_payload: &super::ReplaceProject, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::ReplaceProject(super::ReplaceProject { new_project: base.project.clone() })]

    })())
}
