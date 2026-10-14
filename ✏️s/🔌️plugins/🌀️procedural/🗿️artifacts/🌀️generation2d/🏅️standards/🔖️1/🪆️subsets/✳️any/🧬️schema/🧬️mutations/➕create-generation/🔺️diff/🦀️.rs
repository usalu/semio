//! 🔺️ Sparse diff for `CreateGeneration`, built directly from `(payload, base)`.
use super::CreateGeneration;
use semio_s_artifact_procedural_2d_model::diff::Generation2dGenerationsDelta;
use semio_s_artifact_procedural_2d_model::diff::Generation2dSelectionChange;
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if base.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{}\" already exists.", payload.generation.id), [payload.generation.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.generation.generations.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} generations.", payload.index.unwrap_or_default(), base.generation.generations.len()), [payload.generation.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta::insertion(payload.index.unwrap_or(base.generation.generations.len()), payload.generation.clone())), selected_generation: Some(Generation2dSelectionChange { id: Some(payload.generation.id.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
