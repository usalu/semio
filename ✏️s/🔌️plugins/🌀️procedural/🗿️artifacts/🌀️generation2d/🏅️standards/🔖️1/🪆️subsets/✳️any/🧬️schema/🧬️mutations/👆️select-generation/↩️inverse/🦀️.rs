//! ↩️ `select-generation` inverse — the selection the BASE held, restored as an absolute row; a selection that stays put
//! has nothing to undo.

use crate::select_generation::SelectGeneration;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<SelectGeneration>>(payload: &SelectGeneration, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if base.generation.selected_generation_id == payload.generation_id || payload.generation_id.as_ref().is_some_and(|id| !base.generation.generations.iter().any(|entry| &entry.id == id)) {
        return Ok(Vec::new());
    }
    Ok(vec![Op::from(SelectGeneration { generation_id: base.generation.selected_generation_id.clone() })])
}
