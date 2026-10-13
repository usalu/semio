//! ↩️ Inverse (undo) construction for the `replace-risk` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚠️risks` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceRisk, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.risks.iter().find(|row| row.header.id == payload.risk.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRisk(super::ReplaceRisk { risk: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
