//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_plated_inputs::UpdatePlatedInputs;
use crate::remove_plated_panel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_plated_inputs::UpdatePlatedInputs> + From<crate::remove_plated_panel::RemovePlatedPanel> {
    Ok((|| {
    if let Some(prior) = base.plated_panels.iter().find(|x| x.id == payload.plated_panel.id) {
        vec![Op::from(UpdatePlatedInputs { plated_panel: prior.clone() })]
    } else {
        vec![Op::from(remove_plated_panel::RemovePlatedPanel { index: base.plated_panels.len() })]
    }

    })())
}
