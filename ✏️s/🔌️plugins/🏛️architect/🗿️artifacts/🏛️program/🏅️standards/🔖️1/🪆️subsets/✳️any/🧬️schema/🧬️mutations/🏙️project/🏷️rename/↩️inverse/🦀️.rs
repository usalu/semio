//! ↩️ Inverse (undo) construction for the `rename-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse<Op>(_payload: &super::RenameProject, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_project::RenameProject>,
{
    Ok((|| {
    vec![Op::from(super::RenameProject { new_code: base.project.code.clone() })]

    })())
}
