//! 🔺️ `create-generation` sparse diff construction — delegates the generation-field delta to the
//! existing `semio_framework_artifact_playbook_playbook::GenerationMutation` engine, scoped to a single `Add` op.

use crate::standards::v1::subsets::any::schema::diff::{diff_generation_from_ops, Generation3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration;
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;

/// 🏗️ Builds the sparse generation-field delta for one new generation. `GenerationPlayState` is
/// the document's single flat container, so there is no "unknown owner" case to detect here.
pub fn diff(payload: &CreateGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = &payload.generation.id;
    if base.generation.generations.iter().any(|entry| &entry.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{id}\" already exists."), [id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, &[GenerationMutation::Add { generation: payload.generation.clone() }]))
}
