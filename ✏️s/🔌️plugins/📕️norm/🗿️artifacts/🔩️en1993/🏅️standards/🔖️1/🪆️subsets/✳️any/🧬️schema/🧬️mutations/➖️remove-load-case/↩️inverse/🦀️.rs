use super::RemoveLoadCase;
use crate::mutations::{insert_load_case, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveLoadCase, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.load_cases.len() { return Vec::new(); }
    vec![En1993Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { index: Some(payload.index), load_case: base.load_cases[payload.index].clone()  })]

    })())
}
