//! 🔺️ `change-airtightness-n50` diff — sets the air change rate at 50 Pa n₅₀ in 1/h, refused as a `mutation.invariant` outside the bound its leaf payload schema states.

use super::ChangeAirtightnessN50;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !(payload.new_airtightness_n50.is_finite() && payload.new_airtightness_n50 >= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "airtightness_n50 must be a non-negative finite number.", Vec::<String>::new());
    }
    if base.airtightness_n50 == payload.new_airtightness_n50 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "airtightness_n50 already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { airtightness_n50: Some(payload.new_airtightness_n50), ..Default::default() })
}
