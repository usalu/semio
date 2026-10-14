//! Inverse for `change-fatigue-detail`.
use crate::change_fatigue_detail::ChangeFatigueDetail;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(_payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_detail::ChangeFatigueDetail> {
    Ok((|| {
    vec![Op::from(ChangeFatigueDetail { new_fatigue_detail: base.fatigue_detail.clone() })]

    })())
}
