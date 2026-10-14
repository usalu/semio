//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_through_thickness_inputs::UpdateThroughThicknessInputs;
use crate::remove_section;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_through_thickness_inputs::UpdateThroughThicknessInputs> + From<crate::remove_section::RemoveSection> {
    Ok((|| {
    if let Some(prior) = base.sections.iter().find(|x| x.id == payload.section.id) {
        vec![Op::from(UpdateThroughThicknessInputs { section: prior.clone() })]
    } else {
        vec![Op::from(remove_section::RemoveSection { index: base.sections.len() })]
    }

    })())
}
