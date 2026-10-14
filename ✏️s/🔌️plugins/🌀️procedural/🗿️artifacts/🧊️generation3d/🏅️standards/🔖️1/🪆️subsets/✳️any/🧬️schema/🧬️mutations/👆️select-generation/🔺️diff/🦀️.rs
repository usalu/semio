//! 🔺️ `select-generation` sparse diff — only the play state's selection moves; naming a generation the document does not
//! hold is `mutation.target-missing`, the selection it already holds is `mutation.no-op`.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dSelectionChange;
use crate::select_generation::SelectGeneration;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn diff(payload: &SelectGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if let Some(id) = &payload.generation_id {
        if !base.generation.generations.iter().any(|entry| &entry.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{id}\" does not exist."), [id.clone()]);
        }
    }
    if base.generation.selected_generation_id == payload.generation_id {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", "The generation selection is already as requested.");
    }
    protocol::MutationOutcome::new(Generation3dDiff { selected_generation: Some(Generation3dSelectionChange { id: payload.generation_id.clone() }), ..Default::default() })
}
