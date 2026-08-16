//! 🔺️ Sparse diff construction for the `delete-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::DeleteResource;
use crate::artifacts::program::diff::ProgramResourcesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteResource, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resources: Some(ProgramResourcesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
