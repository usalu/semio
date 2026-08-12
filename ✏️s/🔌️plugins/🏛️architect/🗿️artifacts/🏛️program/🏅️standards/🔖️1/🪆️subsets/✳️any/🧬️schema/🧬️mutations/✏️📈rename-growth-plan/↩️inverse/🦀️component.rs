//! ↩️ Inverse (undo) construction for the `rename-growth-plan` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📈growth` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::RenameGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameGrowthPlan(super::mutation::RenameGrowthPlan { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
