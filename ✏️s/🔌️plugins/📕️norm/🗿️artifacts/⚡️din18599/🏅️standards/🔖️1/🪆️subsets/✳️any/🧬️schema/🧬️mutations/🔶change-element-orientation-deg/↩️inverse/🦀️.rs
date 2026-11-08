//! ↩️ `change-element-orientation-deg` inverse.

use crate::change_element_orientation_deg::ChangeElementOrientationDeg;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementOrientationDeg, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_orientation_deg::ChangeElementOrientationDeg> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementOrientationDeg { element_id: payload.element_id.clone(), new_orientation_deg: row.orientation_deg })]).unwrap_or_default())
}
