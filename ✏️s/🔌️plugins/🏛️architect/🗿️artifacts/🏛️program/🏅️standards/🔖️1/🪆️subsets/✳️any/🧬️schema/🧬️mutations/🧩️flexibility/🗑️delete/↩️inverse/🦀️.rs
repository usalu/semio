//! ↩️ Inverse (undo) construction for the `delete-flexibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧩flexibility` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteFlexibilityRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.flexibility.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateFlexibilityRequirement(super::super::create_flexibility_requirement::CreateFlexibilityRequirement { flexibility_requirement: base.flexibility[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
