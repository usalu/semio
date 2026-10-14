use crate::remove_tension_component::RemoveTensionComponent;
use crate::insert_tension_component;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tension_component::InsertTensionComponent> {
    Ok((|| {
    if payload.index >= base.tension_components.len() { return Vec::new(); }
    vec![Op::from(insert_tension_component::InsertTensionComponent { index: Some(payload.index), tension_component: base.tension_components[payload.index].clone() })]

    })())
}
