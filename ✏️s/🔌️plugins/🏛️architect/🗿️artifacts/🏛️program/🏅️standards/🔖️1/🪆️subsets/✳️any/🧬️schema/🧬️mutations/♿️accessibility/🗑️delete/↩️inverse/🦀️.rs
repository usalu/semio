//! ↩️ Inverse (undo) construction for the `delete-accessibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♿accessibility` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteAccessibilityRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.accessibility.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateAccessibilityRequirement(super::super::create_accessibility_requirement::CreateAccessibilityRequirement { accessibility_requirement: base.accessibility[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
