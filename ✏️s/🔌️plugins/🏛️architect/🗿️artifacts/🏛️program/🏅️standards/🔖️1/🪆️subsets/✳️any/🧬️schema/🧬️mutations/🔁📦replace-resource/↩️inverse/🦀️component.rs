//! ↩️ Inverse (undo) construction for the `replace-resource` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📦resources` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceResource, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resources.iter().find(|row| row.header.id == payload.resource.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceResource(super::mutation::ReplaceResource { resource: existing.clone() })],
        None => Vec::new(),
    }
}
