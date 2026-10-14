//! 🔺️ `delete-generation` sparse diff construction. `FormGeneration` carries no widget/synapse
//! reference, so removing a generation never cascades into the fixture.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationsDelta;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSelectionChange;
use crate::delete_generation::DeleteGeneration;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn diff(payload: &DeleteGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(index) = base.generation.generations.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let selection = (base.generation.selected_generation_id.as_deref() == Some(payload.id.as_str())).then(|| Generation3dSelectionChange { id: base.generation.generations.iter().find(|entry| entry.id != payload.id).map(|entry| entry.id.clone()) });
    protocol::MutationOutcome::new(Generation3dDiff { generations: Some(Generation3dGenerationsDelta::removal(&base.generation.generations, index)), selected_generation: selection, ..Default::default() })
}
