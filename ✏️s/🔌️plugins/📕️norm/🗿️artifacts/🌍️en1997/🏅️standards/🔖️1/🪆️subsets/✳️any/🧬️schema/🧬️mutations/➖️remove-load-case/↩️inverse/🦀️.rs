use crate::remove_load_case::RemoveLoadCase;
use crate::insert_load_case::InsertLoadCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveLoadCase, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_load_case::InsertLoadCase> {
    Ok(match base.footings.iter().find(|row| row.id == payload.footing_id).and_then(|parent| parent.load_cases.get(payload.index).cloned()) {
        Some(load_case) => vec![Op::from(InsertLoadCase { footing_id: payload.footing_id.clone(), index: payload.index, load_case })],
        None => Vec::new(),
    })
}
