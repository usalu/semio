//! ↩️ `select-generation` inverse — the selection the BASE held, restored as an absolute row; a selection that stays put
//! has nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

pub fn inverse(payload: &SelectGeneration, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    if base.generation.selected_generation_id == payload.generation_id || payload.generation_id.as_ref().is_some_and(|id| !base.generation.generations.iter().any(|entry| &entry.id == id)) {
        return Ok(Vec::new());
    }
    Ok(vec![Generation2dMutation::SelectGeneration(SelectGeneration { generation_id: base.generation.selected_generation_id.clone() })])
}
