//! 🔺️ Sparse diff for `CreateGeneration`, built directly from `(payload, base)`.
use super::CreateGeneration;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dGenerationsDelta, Generation2dSelectionChange, insertion_order};
use crate::{Generation2dDiff, Generation2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if base.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{}\" already exists.", payload.generation.id), [payload.generation.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta { added: vec![payload.generation.clone()], reordered: insertion_order(base.generation.generations.iter().map(|entry| entry.id.as_str()), &payload.generation.id, payload.index), ..Default::default() }), selected_generation: Some(Generation2dSelectionChange { id: Some(payload.generation.id.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
