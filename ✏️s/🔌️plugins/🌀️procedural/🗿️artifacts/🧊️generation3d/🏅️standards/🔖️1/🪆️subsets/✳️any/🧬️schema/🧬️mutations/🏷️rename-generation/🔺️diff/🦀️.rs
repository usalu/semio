//! 🔺️ `rename-generation` sparse diff construction. `FormGeneration.name` is a plain display
//! label, not a key (`id` is the only key), so no name-collision Fatal check applies here.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationPatch;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dGenerationsDelta;
use crate::rename_generation::RenameGeneration;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn diff(payload: &RenameGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(existing) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(Generation3dDiff { generations: Some(Generation3dGenerationsDelta { modified: vec![Generation3dGenerationModification { id: payload.id.clone(), patch: Generation3dGenerationPatch { name: Some(payload.new_name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}
