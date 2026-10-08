//! ↩️ Inverse (undo) construction for the `delete-service-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛎️services` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteServiceRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.services.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateServiceRequirement(super::super::create_service_requirement::CreateServiceRequirement { service_requirement: base.services[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
