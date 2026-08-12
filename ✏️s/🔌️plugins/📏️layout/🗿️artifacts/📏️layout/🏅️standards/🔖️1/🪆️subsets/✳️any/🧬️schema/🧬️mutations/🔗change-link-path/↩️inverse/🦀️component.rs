//! ↩ Inverse constructor for `change-link-path` — reconstructed from captured BASE state.

use super::mutation::ChangeLinkPath;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;

//#region 🔗ChangeLinkPath
pub fn inverse_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.links.iter().find(|link| link.id == payload.id) {
        Some(link) => vec![LayoutMutation::ChangeLinkPath(ChangeLinkPath { id: payload.id.clone(), new_path: link.path.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔗ChangeLinkPath
