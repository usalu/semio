//! 🔺️ Sparse diff builder for `ChangeWeight` — upserts the id-keyed `weights` entry.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::{AssemblyModuleWeight, AssemblySnapshot};

pub async fn diff(payload: &super::mutation::ChangeWeight, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.modules.iter().any(|module| module.child_id == payload.module_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Module \"{}\" does not exist.", payload.module_id), [payload.module_id.clone()]);
    }
    if !payload.weight.is_finite() || payload.weight < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weight {} is not a valid non-negative finite value.", payload.weight), [payload.module_id.clone()]);
    }
    if base.weights.iter().any(|weight| weight.module_id == payload.module_id && weight.weight == payload.weight) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Module \"{}\" already has weight {}.", payload.module_id, payload.weight));
    }
    protocol::MutationOutcome::new(AssemblyDiff { weights_upserted: vec![AssemblyModuleWeight { module_id: payload.module_id.clone(), weight: payload.weight }], ..Default::default() })
}
