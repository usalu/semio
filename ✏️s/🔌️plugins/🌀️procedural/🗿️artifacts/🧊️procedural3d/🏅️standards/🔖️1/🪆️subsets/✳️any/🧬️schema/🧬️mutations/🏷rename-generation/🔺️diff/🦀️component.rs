//! 🔺️ `rename-generation` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::rename_generation::mutation::RenameGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &RenameGeneration, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: payload.id.clone(), name: payload.new_name.clone() }])
}
