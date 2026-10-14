//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_stainless_inputs::UpdateStainlessInputs;
use crate::remove_material;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_stainless_inputs::UpdateStainlessInputs> + From<crate::remove_material::RemoveMaterial> {
    Ok((|| {
    if let Some(prior) = base.materials.iter().find(|x| x.id == payload.material.id) {
        vec![Op::from(UpdateStainlessInputs { material: prior.clone() })]
    } else {
        vec![Op::from(remove_material::RemoveMaterial { index: base.materials.len() })]
    }

    })())
}
