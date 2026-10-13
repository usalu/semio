//! ↩️ Inverse (undo) construction for the `rename-resource` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📦resources` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameResource, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.resources.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameResource(super::RenameResource { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }

    })())
}
