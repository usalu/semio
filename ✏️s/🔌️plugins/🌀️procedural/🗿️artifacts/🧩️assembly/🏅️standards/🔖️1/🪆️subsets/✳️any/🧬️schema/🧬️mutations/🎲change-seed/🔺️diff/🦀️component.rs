//! 🔺️ Sparse diff builder for `ChangeSeed` — a single-field scalar delta.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;

pub async fn diff(payload: &super::mutation::ChangeSeed, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(AssemblyDiff { seed: Some(payload.seed), ..Default::default() })
}
