//! ↩️ Inverse (undo) construction for the `delete-human-factor-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧠human-factors` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteHumanFactorRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.human_factors.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateHumanFactorRequirement(super::super::create_human_factor_requirement::CreateHumanFactorRequirement { human_factor_requirement: base.human_factors[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
