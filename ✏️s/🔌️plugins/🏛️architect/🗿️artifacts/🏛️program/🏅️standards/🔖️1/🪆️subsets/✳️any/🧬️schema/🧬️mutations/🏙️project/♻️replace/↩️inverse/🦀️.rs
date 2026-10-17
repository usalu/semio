//! ↩️ Inverse (undo) construction for the `replace-project` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📁update-project` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

pub fn inverse<Op>(_payload: &super::ReplaceProject, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_project::ReplaceProject>,
{
    Ok((|| {
    vec![Op::from(super::ReplaceProject { new_project: base.project.clone() })]

    })())
}
