//! 🔺️ `change-airtightness-n50` diff.

use super::ChangeAirtightnessN50;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_airtightness_n50.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "airtightness_n50 must be a finite number.", Vec::<String>::new());
    }
    if base.airtightness_n50 == payload.new_airtightness_n50 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "airtightness_n50 already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { airtightness_n50: Some(payload.new_airtightness_n50), ..Default::default() })
}
