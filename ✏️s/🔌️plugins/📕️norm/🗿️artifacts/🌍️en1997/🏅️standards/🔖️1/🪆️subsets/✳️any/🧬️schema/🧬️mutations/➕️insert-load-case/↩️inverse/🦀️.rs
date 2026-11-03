use crate::insert_load_case::InsertLoadCase;
use crate::remove_load_case::RemoveLoadCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertLoadCase, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_load_case::RemoveLoadCase> {
    let at = base.footings.iter().find(|row| row.id == payload.footing_id).map_or(0, |parent| payload.index.min(parent.load_cases.len()));
    Ok(vec![Op::from(RemoveLoadCase { footing_id: payload.footing_id.clone(), index: at })])
}
