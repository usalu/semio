//! ↩️ Inverse (undo) construction for the `replace-workshop` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎓workshops` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceWorkshop, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.workshops.iter().find(|row| row.header.id == payload.workshop.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceWorkshop(super::ReplaceWorkshop { workshop: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
