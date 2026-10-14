//! ↩️ `change-fatigue-details` inverse.

use crate::change_fatigue_details::ChangeFatigueDetails;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeFatigueDetails, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_details::ChangeFatigueDetails> {
    Ok((|| {
    vec![Op::from(ChangeFatigueDetails { fatigue_details: base.fatigue_details.clone() })]

    })())
}
