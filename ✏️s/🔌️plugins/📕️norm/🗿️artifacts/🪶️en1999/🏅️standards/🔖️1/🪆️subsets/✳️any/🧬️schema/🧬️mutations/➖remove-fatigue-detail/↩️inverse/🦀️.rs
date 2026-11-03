//! ↩️ `remove-fatigue-detail` inverse.

use crate::remove_fatigue_detail::RemoveFatigueDetail;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_fatigue_detail;

pub fn inverse<Op>(payload: &RemoveFatigueDetail, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_detail::InsertFatigueDetail> {
    Ok((|| {
    if let Some((index, row)) = base.fatigue_details.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_fatigue_detail::InsertFatigueDetail { index: index as u32, fatigue_detail: row.clone() })]
    } else { Vec::new() }

    })())
}
