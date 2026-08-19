//! ↩️ Inverse for `ChangeWeight` — restores the PRIOR weight from a real BASE lookup if one existed;
//! if `change-weight` inserted a fresh row (no prior entry), its true undo is `remove-weight`, not a
//! same-kind change — a genuine removal, not a lossy approximation.
use crate::artifacts::assembly::mutations::{change_weight, remove_weight, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn inverse(payload: &super::mutation::ChangeWeight, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    match base.weights.iter().find(|weight| weight.module_id == payload.module_id) {
        Some(prior) => vec![change_weight(payload.module_id.clone(), prior.weight)],
        None => vec![remove_weight(payload.module_id.clone())],
    }
}
