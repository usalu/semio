//! ↩️ Inverse (undo) construction for the `rename-service-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛎️services` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameServiceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.services.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameServiceRequirement(super::RenameServiceRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
