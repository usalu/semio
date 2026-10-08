//! 🔺️ `change-fatigue-details` diff.

use crate::mutations::change_fatigue_details::ChangeFatigueDetails;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999FatigueDetailsRows};

pub fn diff(payload: &ChangeFatigueDetails, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.fatigue_details == &payload.fatigue_details {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.fatigue_details.iter().enumerate().find(|(at, row)| payload.fatigue_details[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::setting(&base.fatigue_details, &payload.fatigue_details)), ..Default::default() })
}
