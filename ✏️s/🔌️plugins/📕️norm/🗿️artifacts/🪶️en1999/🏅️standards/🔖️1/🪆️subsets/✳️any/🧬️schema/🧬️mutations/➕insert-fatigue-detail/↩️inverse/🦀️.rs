//! ↩️ `insert-fatigue-detail` inverse.

use crate::insert_fatigue_detail::InsertFatigueDetail;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_fatigue_detail;

pub fn inverse<Op>(payload: &InsertFatigueDetail, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_detail::RemoveFatigueDetail> {
    Ok((|| {
    vec![Op::from(remove_fatigue_detail::RemoveFatigueDetail { id: payload.fatigue_detail.id.clone() })]

    })())
}
