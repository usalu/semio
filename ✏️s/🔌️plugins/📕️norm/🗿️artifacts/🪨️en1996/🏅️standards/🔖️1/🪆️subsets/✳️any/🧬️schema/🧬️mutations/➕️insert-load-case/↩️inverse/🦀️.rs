use crate::insert_load_case::InsertLoadCase;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::remove_load_case;
pub fn inverse<Op>(payload: &InsertLoadCase, _base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_load_case::RemoveLoadCase> {
    Ok((|| {
    vec![Op::from(remove_load_case::RemoveLoadCase { wall_index: payload.wall_index, index: payload.index })]

    })())
}
