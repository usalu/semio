//! 🔺️ `delete-generation` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::delete_generation::mutation::DeleteGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &DeleteGeneration, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }])
}
