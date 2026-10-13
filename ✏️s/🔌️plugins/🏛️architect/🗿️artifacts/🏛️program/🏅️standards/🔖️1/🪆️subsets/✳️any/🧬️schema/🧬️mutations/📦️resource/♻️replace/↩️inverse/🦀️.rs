//! ↩️ Inverse (undo) construction for the `replace-resource` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📦resources` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceResource, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.resources.iter().find(|row| row.header.id == payload.resource.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceResource(super::ReplaceResource { resource: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
