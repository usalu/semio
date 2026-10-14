//! ↩️ `delete-generation` inverse — reconstructs a `create-generation` from BASE state; a
//! generation already absent from `base` has nothing to undo.

use crate::create_generation::CreateGeneration;
use crate::delete_generation::DeleteGeneration;
use crate::select_generation::SelectGeneration;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse(payload: &DeleteGeneration, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.generation.generations.iter().position(|entry| entry.id == payload.id).map(|at| {
        let mut steps = Vec::new();
        if base.generation.selected_generation_id.as_deref() != Some(payload.id.as_str()) {
            steps.push(Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: base.generation.selected_generation_id.clone() }));
        }
        steps.push(Generation3dMutation::CreateGeneration(CreateGeneration { generation: base.generation.generations[at].clone(), index: Some(at) }));
        steps
    }).unwrap_or_default()

    })())
}
