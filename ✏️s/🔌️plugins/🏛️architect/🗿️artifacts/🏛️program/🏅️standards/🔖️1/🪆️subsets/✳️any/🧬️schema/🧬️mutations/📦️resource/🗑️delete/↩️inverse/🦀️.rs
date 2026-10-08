//! ↩️ Inverse (undo) construction for the `delete-resource` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📦resources` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteResource, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.resources.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateResource(super::super::create_resource::CreateResource { resource: base.resources[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
