//! ↩️ Inverse (undo) construction for the `replace-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceOptionEvaluation, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOptionEvaluation(super::ReplaceOptionEvaluation { option_evaluation: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
