//! 🔺️ Sparse diff builder for `ChangeSeed` — a single-field scalar delta.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn diff(payload: &super::ChangeSeed, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("wfc3d.seed.unchanged", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Wfc3dDiff { seed: Some(payload.seed), ..Default::default() })
}
