//! ↩️ Inverse (undo) construction for the `update_meta` mutation leaf — computed from captured
//! pre-state.

use super::mutation::{RenameMeta, ReplaceMeta};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state title.
pub fn inverse_rename(_payload: &RenameMeta, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameMeta(RenameMeta { new_title: base.meta.title.clone() })]
}

/// ↩️ Undo by restoring the pre-state meta wholesale.
pub fn inverse_replace(_payload: &ReplaceMeta, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceMeta(ReplaceMeta { new_meta: base.meta.clone() })]
}
