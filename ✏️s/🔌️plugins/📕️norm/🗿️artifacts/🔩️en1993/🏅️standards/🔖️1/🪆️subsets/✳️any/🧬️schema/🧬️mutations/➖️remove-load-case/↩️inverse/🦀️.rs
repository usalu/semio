use crate::remove_load_case::RemoveLoadCase;
use crate::insert_load_case;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveLoadCase, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_load_case::InsertLoadCase> {
    Ok((|| {
    if payload.index >= base.load_cases.len() { return Vec::new(); }
    vec![Op::from(insert_load_case::InsertLoadCase { index: Some(payload.index), load_case: base.load_cases[payload.index].clone()  })]

    })())
}
