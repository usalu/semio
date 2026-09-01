//! 🔺️ Sparse diff builder for `RemoveWeight` — removes the id from `weights`.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub fn diff(payload: &super::RemoveWeight, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if !base.weights.iter().any(|weight| weight.module_id == payload.module_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Module \"{}\" has no weight override.", payload.module_id), [payload.module_id.clone()]);
    }
    protocol::MutationOutcome::new(AssemblyDiff { weights_removed: vec![payload.module_id.clone()], ..Default::default() })
}
