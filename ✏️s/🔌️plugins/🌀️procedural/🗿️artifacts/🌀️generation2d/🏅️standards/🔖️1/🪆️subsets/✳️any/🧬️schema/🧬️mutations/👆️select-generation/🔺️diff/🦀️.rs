//! 🔺️ `select-generation` sparse diff — only the play state's selection moves; naming a generation the document does not
//! hold is `mutation.target-missing`, the selection it already holds is `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dSelectionChange};
use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use crate::Generation2dSnapshot;

pub fn diff(payload: &SelectGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if let Some(id) = &payload.generation_id {
        if !base.generation.generations.iter().any(|entry| &entry.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{id}\" does not exist."), [id.clone()]);
        }
    }
    if base.generation.selected_generation_id == payload.generation_id {
        return protocol::MutationOutcome::new(Generation2dDiff::default()).warning("mutation.no-op", "The generation selection is already as requested.");
    }
    protocol::MutationOutcome::new(Generation2dDiff { selected_generation: Some(Generation2dSelectionChange { id: payload.generation_id.clone() }), ..Default::default() })
}
