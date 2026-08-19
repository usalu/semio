//! ↩ Inverse constructor for `delete-link` — captures the removed link's full payload and position.

use super::mutation::DeleteLink;
use crate::artifacts::layout::mutations::{create_link, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 🗑️DeleteLink
pub async fn inverse_delete_link(payload: &DeleteLink, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.links.iter().position(|link| link.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateLink(create_link::mutation::CreateLink { link: base.links[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteLink
