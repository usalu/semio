//! 🔺️ Sparse diff for `DeleteGeneration`, built directly from `(payload, base)`.
use super::DeleteGeneration;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dGenerationsDelta, Generation2dSelectionChange};
use crate::{Generation2dDiff, Generation2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let selection = (base.generation.selected_generation_id.as_deref() == Some(payload.id.as_str())).then(|| Generation2dSelectionChange { id: base.generation.generations.iter().find(|entry| entry.id != payload.id).map(|entry| entry.id.clone()) });
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), selected_generation: selection, ..Default::default() })
}
//#endregion 🔖️Diff
