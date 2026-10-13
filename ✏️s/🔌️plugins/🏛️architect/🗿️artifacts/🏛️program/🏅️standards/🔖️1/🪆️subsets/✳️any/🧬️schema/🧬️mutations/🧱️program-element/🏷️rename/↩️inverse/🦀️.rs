//! ↩️ Inverse (undo) construction for the `rename-program-element` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧱elements` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameProgramElement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.elements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameProgramElement(super::RenameProgramElement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }

    })())
}
