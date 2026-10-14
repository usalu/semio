//! ↩️ `change-element-area` inverse — restores the element's `area_m2`, computed from BASE state; a missing target yields no step.

use crate::change_element_area::ChangeElementArea;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeElementArea, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_area::ChangeElementArea> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(ChangeElementArea { element_id: payload.element_id.clone(), new_area_m2: element.area_m2 })]).unwrap_or_default()

    })())
}
