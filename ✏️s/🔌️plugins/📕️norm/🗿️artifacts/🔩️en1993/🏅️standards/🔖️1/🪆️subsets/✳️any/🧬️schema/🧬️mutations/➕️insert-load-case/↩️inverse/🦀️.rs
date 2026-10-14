use crate::insert_load_case::InsertLoadCase;
use crate::remove_load_case;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertLoadCase, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_load_case::RemoveLoadCase> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.load_cases.len());
    vec![Op::from(remove_load_case::RemoveLoadCase { index: at  })]

    })())
}
