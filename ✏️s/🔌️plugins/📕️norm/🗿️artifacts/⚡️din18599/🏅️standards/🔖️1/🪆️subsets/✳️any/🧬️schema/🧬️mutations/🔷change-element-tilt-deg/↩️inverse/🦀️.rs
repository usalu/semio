//! ↩️ `change-element-tilt-deg` inverse.

use crate::change_element_tilt_deg::ChangeElementTiltDeg;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementTiltDeg, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_tilt_deg::ChangeElementTiltDeg> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementTiltDeg { element_id: payload.element_id.clone(), new_tilt_deg: row.tilt_deg })]).unwrap_or_default())
}
