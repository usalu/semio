//! ↩️ Inverse (undo) construction for the `replace-site-context` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📍site-context` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceSiteContext, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSiteContext(super::ReplaceSiteContext { site_context: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
