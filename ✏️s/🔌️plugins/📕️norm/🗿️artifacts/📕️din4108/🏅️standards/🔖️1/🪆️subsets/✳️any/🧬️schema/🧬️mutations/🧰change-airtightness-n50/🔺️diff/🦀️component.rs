//! 🔺️ `change-airtightness-n50` — sparse diff construction.

use super::mutation::ChangeAirtightnessN50;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_airtightness_n50.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Airtightness n50 must be a finite number.", Vec::<String>::new());
    }
    if base.airtightness_n50 == payload.new_airtightness_n50 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Airtightness n50 already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { airtightness_n50: Some(payload.new_airtightness_n50.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
