use crate::remove_fatigue_detail::RemoveFatigueDetail;
use crate::insert_fatigue_detail;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFatigueDetail, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_detail::InsertFatigueDetail> {
    Ok((|| {
    if payload.index >= base.fatigue_details.len() { return Vec::new(); }
    vec![Op::from(insert_fatigue_detail::InsertFatigueDetail { index: Some(payload.index), fatigue_detail: base.fatigue_details[payload.index].clone() })]

    })())
}
