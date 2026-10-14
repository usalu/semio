//! ↩️ `change-element-inclination-deg` inverse — restores the element's `inclination_deg`, computed from BASE state; a missing target yields no step.

use crate::change_element_inclination_deg::ChangeElementInclinationDeg;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeElementInclinationDeg, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_inclination_deg::ChangeElementInclinationDeg> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(ChangeElementInclinationDeg { element_id: payload.element_id.clone(), new_inclination_deg: element.inclination_deg })]).unwrap_or_default()

    })())
}
