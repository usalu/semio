//! ↩️ `select-generation` inverse — the selection the BASE held, restored as an absolute row; a selection that stays put
//! has nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(payload: &SelectGeneration, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    if base.generation.selected_generation_id == payload.generation_id || payload.generation_id.as_ref().is_some_and(|id| !base.generation.generations.iter().any(|entry| &entry.id == id)) {
        return Ok(Vec::new());
    }
    Ok(vec![Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: base.generation.selected_generation_id.clone() })])
}
