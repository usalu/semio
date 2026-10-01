//! ↩️ Inverse (undo) construction for the `rename-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameFlowRequirement(super::RenameFlowRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
