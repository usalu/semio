//! ↩️ Inverse for `RemoveWeight` — restores the prior entry from a real BASE lookup (missing id ⇒
//! empty: no-op, nothing to undo).

use crate::artifacts::assembly::mutations::{change_weight, AssemblyMutation};
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn inverse(payload: &super::mutation::RemoveWeight, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
    match base.weights.iter().find(|weight| weight.module_id == payload.module_id) {
        Some(prior) => vec![change_weight(payload.module_id.clone(), prior.weight)],
        None => Vec::new(),
    }
}
