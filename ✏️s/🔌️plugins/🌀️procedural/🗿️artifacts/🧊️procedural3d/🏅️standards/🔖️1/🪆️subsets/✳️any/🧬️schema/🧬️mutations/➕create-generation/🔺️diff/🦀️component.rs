//! 🔺️ `create-generation` sparse diff construction — delegates the generation-field delta to the
//! existing `flow::playbook::GenerationMutation` engine, scoped to a single `Add` op.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::create_generation::mutation::CreateGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

/// 🏗️ Builds the sparse generation-field delta for one new generation.
pub fn diff(payload: &CreateGeneration, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::Add { generation: payload.generation.clone() }])
}
