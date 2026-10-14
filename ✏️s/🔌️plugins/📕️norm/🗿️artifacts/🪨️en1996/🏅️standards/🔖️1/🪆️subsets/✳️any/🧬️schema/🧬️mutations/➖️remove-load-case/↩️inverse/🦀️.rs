use crate::remove_load_case::RemoveLoadCase;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use crate::insert_load_case;
pub fn inverse<Op>(payload: &RemoveLoadCase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_load_case::InsertLoadCase> {
    Ok((|| {
    if payload.wall_index >= base.walls.len() || payload.index >= base.walls[payload.wall_index].load_cases.len() { Vec::new() } else { vec![Op::from(insert_load_case::InsertLoadCase { wall_index: payload.wall_index, index: payload.index, load_case: base.walls[payload.wall_index].load_cases[payload.index].clone() })] }

    })())
}
