//! ↩️ Inverse (undo) construction for the `rename-meta` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏷️update-meta` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state title.
pub fn inverse<Op>(_payload: &super::RenameMeta, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_meta::RenameMeta>,
{
    Ok((|| {
    vec![Op::from(super::RenameMeta { new_title: base.meta.title.clone() })]

    })())
}
