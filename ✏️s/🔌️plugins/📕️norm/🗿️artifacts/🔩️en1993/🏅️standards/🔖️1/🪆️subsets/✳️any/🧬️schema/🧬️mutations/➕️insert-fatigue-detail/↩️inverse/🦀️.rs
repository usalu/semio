use crate::insert_fatigue_detail::InsertFatigueDetail;
use crate::remove_fatigue_detail;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_detail::RemoveFatigueDetail> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.fatigue_details.len());
    vec![Op::from(remove_fatigue_detail::RemoveFatigueDetail { index: at })]

    })())
}
