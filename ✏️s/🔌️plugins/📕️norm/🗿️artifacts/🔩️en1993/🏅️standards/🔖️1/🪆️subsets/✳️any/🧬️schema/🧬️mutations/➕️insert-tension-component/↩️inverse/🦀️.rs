use crate::insert_tension_component::InsertTensionComponent;
use crate::remove_tension_component;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertTensionComponent, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tension_component::RemoveTensionComponent> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.tension_components.len());
    vec![Op::from(remove_tension_component::RemoveTensionComponent { index: at })]

    })())
}
