//! ↩️ Inverse (undo) construction for the `create-site-context` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📍site-context` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateSiteContext, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSiteContext(super::super::delete_site_context::mutation::DeleteSiteContext { id: payload.site_context.header.id.clone() })]
}
