//! ↩️ Inverse (undo) construction for the `delete-resource` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📦resources` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteResource, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resources.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateResource(super::super::create_resource::CreateResource { resource: existing.clone() })],
        None => Vec::new(),
    }
}
