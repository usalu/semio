use super::RemoveFatigueDetail;
use crate::mutations::{insert_fatigue_detail, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveFatigueDetail, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.fatigue_details.len() { return Vec::new(); }
    vec![En1993Mutation::InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail { index: Some(payload.index), fatigue_detail: base.fatigue_details[payload.index].clone() })]

    })())
}
