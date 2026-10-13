//! ↩️ Inverse (undo) construction for the `rename-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse(_payload: &super::RenameProject, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::RenameProject(super::RenameProject { new_code: base.project.code.clone() })]

    })())
}
