//! ↩️ Inverse (undo) construction for the `delete-site-context` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📍site-context` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteSiteContext, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.site_context.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSiteContext(super::super::create_site_context::mutation::CreateSiteContext { site_context: existing.clone() })],
        None => Vec::new(),
    }
}
