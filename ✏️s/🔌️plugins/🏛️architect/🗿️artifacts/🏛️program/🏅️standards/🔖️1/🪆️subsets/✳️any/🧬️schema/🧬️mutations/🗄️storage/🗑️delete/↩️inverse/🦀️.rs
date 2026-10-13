//! ↩️ Inverse (undo) construction for the `delete-storage-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗄️storage` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteStorageRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.storage.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateStorageRequirement(super::super::create_storage_requirement::CreateStorageRequirement { storage_requirement: base.storage[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
