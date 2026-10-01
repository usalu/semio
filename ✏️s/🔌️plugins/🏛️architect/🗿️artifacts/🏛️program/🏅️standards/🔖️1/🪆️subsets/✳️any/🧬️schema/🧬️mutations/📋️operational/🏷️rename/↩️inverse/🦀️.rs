//! ↩️ Inverse (undo) construction for the `rename-operational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📋operations` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameOperationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.operations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameOperationalRequirement(super::RenameOperationalRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
