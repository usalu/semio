use super::RemoveTensionComponent;
use crate::mutations::{insert_tension_component, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.tension_components.len() { return Vec::new(); }
    vec![En1993Mutation::InsertTensionComponent(insert_tension_component::InsertTensionComponent { index: Some(payload.index), tension_component: base.tension_components[payload.index].clone() })]

    })())
}
