//! 🔺️ `change-fatigue-details` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_fatigue_details::ChangeFatigueDetails;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeFatigueDetails, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.fatigue_details == &payload.fatigue_details {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(payload.fatigue_details.clone()), ..Default::default() })
}
