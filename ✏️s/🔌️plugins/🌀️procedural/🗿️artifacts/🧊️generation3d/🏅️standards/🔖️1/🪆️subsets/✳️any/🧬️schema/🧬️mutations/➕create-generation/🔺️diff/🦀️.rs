//! 🔺️ `create-generation` sparse diff construction — delegates the generation-field delta to the
//! existing `semio_framework_artifact_playbook_playbook::GenerationMutation` engine, scoped to a single `Add` op.

use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dGenerationsDelta, Generation3dSelectionChange};
use crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse generation-field delta for one new generation. `GenerationPlayState` is
/// the document's single flat container, so there is no "unknown owner" case to detect here.
pub fn diff(payload: &CreateGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = &payload.generation.id;
    if base.generation.generations.iter().any(|entry| &entry.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{id}\" already exists."), [id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.generation.generations.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} generations.", payload.index.unwrap_or_default(), base.generation.generations.len()), [payload.generation.id.clone()]);
    }
    protocol::MutationOutcome::new(Generation3dDiff { generations: Some(Generation3dGenerationsDelta::insertion(payload.index.unwrap_or(base.generation.generations.len()), payload.generation.clone())), selected_generation: Some(Generation3dSelectionChange { id: Some(payload.generation.id.clone()) }), ..Default::default() })
}
