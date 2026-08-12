//! ↩️ Inverse (undo) construction for the `rename-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::RenameWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameWayfindingRequirement(super::mutation::RenameWayfindingRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
