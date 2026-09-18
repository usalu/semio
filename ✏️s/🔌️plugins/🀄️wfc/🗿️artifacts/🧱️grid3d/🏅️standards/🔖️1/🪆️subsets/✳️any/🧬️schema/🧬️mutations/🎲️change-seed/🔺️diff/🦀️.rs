//! 🔺️ Sparse diff builder for `ChangeSeed` — an id-keyed delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;

pub fn diff(payload: &super::ChangeSeed, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(Grid3dDiff { seed: Some(payload.seed), ..Default::default() })
}
