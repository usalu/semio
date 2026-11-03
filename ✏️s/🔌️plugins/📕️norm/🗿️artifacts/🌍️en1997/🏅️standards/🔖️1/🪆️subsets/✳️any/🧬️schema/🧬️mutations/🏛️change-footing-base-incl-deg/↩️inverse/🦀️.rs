//! ↩️ `change-footing-base-incl-deg` inverse: restores the previous `base_inclination_deg`.

use crate::change_footing_base_incl_deg::ChangeFootingBaseInclDeg;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeFootingBaseInclDeg, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footing_base_incl_deg::ChangeFootingBaseInclDeg> {
    Ok(match base.footings.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFootingBaseInclDeg { id: payload.id.clone(), new_base_inclination_deg: row.base_inclination_deg })],
        None => Vec::new(),
    })
}
