//! ↩️ Inverse (undo) construction for the `delete-regulatory-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📜regulatory` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteRegulatoryRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.regulatory.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRegulatoryRequirement(super::super::create_regulatory_requirement::mutation::CreateRegulatoryRequirement { regulatory_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
