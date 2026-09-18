//! 🔺️ Sparse diff builder for `ChangeSeed` — one scalar lane, every collection lane untouched.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangeSeed, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Grid2dDiff { seed: Some(payload.seed), ..Default::default() })
}
