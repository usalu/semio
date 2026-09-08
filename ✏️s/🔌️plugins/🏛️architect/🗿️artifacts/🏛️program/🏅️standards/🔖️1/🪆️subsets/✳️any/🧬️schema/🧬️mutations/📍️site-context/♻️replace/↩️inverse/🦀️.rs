//! ↩️ Inverse (undo) construction for the `replace-site-context` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📍site-context` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceSiteContext, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSiteContext(super::ReplaceSiteContext { site_context: existing.clone() })],
        None => Vec::new(),
    }
}
