//! ↩️ Inverse (undo) construction for the `replace-flow-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌊flows` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.flow_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceFlowRequirement(super::mutation::ReplaceFlowRequirement { flow_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
