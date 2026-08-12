//! 🔺️ Sparse diff construction for the `create-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::CreateResource;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramResourcesDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.resources` on apply.
pub fn diff(payload: &CreateResource, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { resources: Some(ProgramResourcesDelta { added: vec![payload.resource.clone()], ..Default::default() }), ..Default::default() }
}
