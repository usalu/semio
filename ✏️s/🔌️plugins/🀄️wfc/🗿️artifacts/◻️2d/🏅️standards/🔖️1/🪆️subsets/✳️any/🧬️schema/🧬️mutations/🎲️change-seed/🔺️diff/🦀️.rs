//! 🔺️ Sparse diff builder for `ChangeSeed` — a real id-keyed delta, never a whole-snapshot capture.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn diff(payload: &super::ChangeSeed, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Wfc2dDiff { seed: Some(payload.seed), ..Default::default() })
}
