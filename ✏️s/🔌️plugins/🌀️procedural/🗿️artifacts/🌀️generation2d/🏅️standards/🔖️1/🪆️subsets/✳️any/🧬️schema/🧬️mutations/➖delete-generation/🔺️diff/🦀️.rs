//! 🔺️ Sparse diff for `DeleteGeneration`, built directly from `(payload, base)`.
use super::DeleteGeneration;
use semio_s_artifact_procedural_2d_model::diff::Generation2dGenerationsDelta;
use semio_s_artifact_procedural_2d_model::diff::Generation2dSelectionChange;
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(index) = base.generation.generations.iter().position(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let selection = (base.generation.selected_generation_id.as_deref() == Some(payload.id.as_str())).then(|| Generation2dSelectionChange { id: base.generation.generations.iter().find(|entry| entry.id != payload.id).map(|entry| entry.id.clone()) });
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta::removal(&base.generation.generations, index)), selected_generation: selection, ..Default::default() })
}
//#endregion 🔖️Diff
