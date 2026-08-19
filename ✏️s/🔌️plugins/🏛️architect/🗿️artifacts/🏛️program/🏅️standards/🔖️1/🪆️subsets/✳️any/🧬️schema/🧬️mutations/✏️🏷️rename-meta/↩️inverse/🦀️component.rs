//! ↩️ Inverse (undo) construction for the `rename-meta` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏷️update-meta` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state title.
pub async fn inverse(_payload: &super::mutation::RenameMeta, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameMeta(super::mutation::RenameMeta { new_title: base.meta.title.clone() })]
}
