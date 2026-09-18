//! 🔺️ Sparse diff builder for `ChangeSeed` — a single-field scalar delta.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;

pub fn diff(payload: &super::ChangeSeed, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if base.seed == payload.seed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seed is already {}.", payload.seed));
    }
    protocol::MutationOutcome::new(BitmapDiff { seed: Some(payload.seed), ..Default::default() })
}
