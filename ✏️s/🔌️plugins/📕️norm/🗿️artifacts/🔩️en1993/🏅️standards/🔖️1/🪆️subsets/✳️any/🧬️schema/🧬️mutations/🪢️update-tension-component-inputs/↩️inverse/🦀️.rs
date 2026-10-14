//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_tension_component_inputs::UpdateTensionComponentInputs;
use crate::remove_tension_component;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_tension_component_inputs::UpdateTensionComponentInputs> + From<crate::remove_tension_component::RemoveTensionComponent> {
    Ok((|| {
    if let Some(prior) = base.tension_components.iter().find(|x| x.id == payload.tension_component.id) {
        vec![Op::from(UpdateTensionComponentInputs { tension_component: prior.clone() })]
    } else {
        vec![Op::from(remove_tension_component::RemoveTensionComponent { index: base.tension_components.len() })]
    }

    })())
}
