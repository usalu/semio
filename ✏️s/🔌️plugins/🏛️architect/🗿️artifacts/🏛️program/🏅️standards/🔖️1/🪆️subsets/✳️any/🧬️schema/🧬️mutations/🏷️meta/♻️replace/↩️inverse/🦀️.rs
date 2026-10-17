//! ↩️ Inverse (undo) construction for the `replace-meta` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏷️update-meta` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state meta wholesale.
pub fn inverse<Op>(_payload: &super::ReplaceMeta, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_meta::ReplaceMeta>,
{
    Ok((|| {
    vec![Op::from(super::ReplaceMeta { new_meta: base.meta.clone() })]

    })())
}
