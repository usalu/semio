//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_hss_inputs::UpdateHssInputs;
use crate::remove_load_case;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateHssInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_hss_inputs::UpdateHssInputs> + From<crate::remove_load_case::RemoveLoadCase> {
    Ok((|| {
    if let Some(prior) = base.load_cases.iter().find(|x| x.id == payload.load_case.id) {
        vec![Op::from(UpdateHssInputs { load_case: prior.clone()  })]
    } else {
        vec![Op::from(remove_load_case::RemoveLoadCase { index: base.load_cases.len()  })]
    }

    })())
}
