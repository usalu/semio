//! ↩️ Inverse (undo) construction for the `replace-meta` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏷️update-meta` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo by restoring the pre-state meta wholesale.
pub async fn inverse(_payload: &super::mutation::ReplaceMeta, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceMeta(super::mutation::ReplaceMeta { new_meta: base.meta.clone() })]
}
