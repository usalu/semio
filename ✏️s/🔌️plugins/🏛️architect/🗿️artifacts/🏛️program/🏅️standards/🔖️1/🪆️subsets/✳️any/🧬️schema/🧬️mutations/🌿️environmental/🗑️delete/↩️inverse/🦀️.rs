//! ↩️ Inverse (undo) construction for the `delete-environmental-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌿environmental` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteEnvironmentalRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.environmental.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateEnvironmentalRequirement(super::super::create_environmental_requirement::CreateEnvironmentalRequirement { environmental_requirement: base.environmental[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
