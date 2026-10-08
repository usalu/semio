use super::InsertLoadCase;
use crate::mutations::{remove_load_case, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertLoadCase, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.load_cases.len());
    vec![En1993Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { index: at  })]

    })())
}
