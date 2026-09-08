//! ↩️ Inverse (undo) construction for the `rename-meta` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏷️update-meta` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state title.
pub fn inverse(_payload: &super::RenameMeta, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameMeta(super::RenameMeta { new_title: base.meta.title.clone() })]
}
