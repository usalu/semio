//! ↩️ `change-fatigue-details` inverse.

use crate::mutations::change_fatigue_details::ChangeFatigueDetails;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeFatigueDetails, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeFatigueDetails(ChangeFatigueDetails { fatigue_details: base.fatigue_details.clone() })]
}
