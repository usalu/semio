//! ↩️ `change-fatigue-details` inverse.

use crate::mutations::change_fatigue_details::ChangeFatigueDetails;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeFatigueDetails, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1999Mutation::ChangeFatigueDetails(ChangeFatigueDetails { fatigue_details: base.fatigue_details.clone() })]

    })())
}
