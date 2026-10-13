//! ↩️ Inverse (undo) construction for the `rename-risk` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚠️risks` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameRisk, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.risks.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameRisk(super::RenameRisk { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }

    })())
}
