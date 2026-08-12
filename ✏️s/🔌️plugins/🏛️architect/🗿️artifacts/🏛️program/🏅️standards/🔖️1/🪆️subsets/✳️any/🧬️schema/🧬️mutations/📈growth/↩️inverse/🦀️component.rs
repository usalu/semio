//! ↩️ Inverse (undo) construction for the `growth` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateGrowthPlan, DeleteGrowthPlan, RenameGrowthPlan, ReplaceGrowthPlan};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateGrowthPlan, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteGrowthPlan(DeleteGrowthPlan { id: payload.growth_plan.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateGrowthPlan(CreateGrowthPlan { growth_plan: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameGrowthPlan(RenameGrowthPlan { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.growth_plan.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceGrowthPlan(ReplaceGrowthPlan { growth_plan: existing.clone() })],
        None => Vec::new(),
    }
}
