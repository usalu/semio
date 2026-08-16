//! 🔺️ Sparse diff construction for the `rename-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::RenameResource;
use crate::artifacts::program::diff::{ProgramResourcesDelta, ProgramResourcesPatchEntry};
use crate::artifacts::program::registers::ResourcePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameResource, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ResourcePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
