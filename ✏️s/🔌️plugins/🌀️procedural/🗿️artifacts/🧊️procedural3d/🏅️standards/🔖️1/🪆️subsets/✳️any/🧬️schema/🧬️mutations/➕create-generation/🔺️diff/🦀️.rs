//! 🔺️ `create-generation` sparse diff construction — delegates the generation-field delta to the
//! existing `flow::playbook::GenerationMutation` engine, scoped to a single `Add` op.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::create_generation::CreateGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

/// 🏗️ Builds the sparse generation-field delta for one new generation. `GenerationPlayState` is
/// the document's single flat container, so there is no "unknown owner" case to detect here.
pub fn diff(payload: &CreateGeneration, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let id = &payload.generation.id;
    if base.generation.generations.iter().any(|entry| &entry.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{id}\" already exists."), [id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Add { generation: payload.generation.clone() }]))
}
