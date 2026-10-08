//! 🔺️ `select-generation` sparse diff — only the play state's selection moves; naming a generation the document does not
//! hold is `mutation.target-missing`, the selection it already holds is `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::diff::{diff_generation_with, Generation3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use crate::Generation3dSnapshot;

pub fn diff(payload: &SelectGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if let Some(id) = &payload.generation_id {
        if !base.generation.generations.iter().any(|entry| &entry.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{id}\" does not exist."), [id.clone()]);
        }
    }
    if base.generation.selected_generation_id == payload.generation_id {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", "The generation selection is already as requested.");
    }
    protocol::MutationOutcome::new(diff_generation_with(base, |generation| generation.selected_generation_id.clone_from(&payload.generation_id)))
}
