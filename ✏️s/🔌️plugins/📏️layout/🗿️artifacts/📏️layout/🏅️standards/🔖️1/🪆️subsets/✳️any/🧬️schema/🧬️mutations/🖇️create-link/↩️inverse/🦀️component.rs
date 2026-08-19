//! ↩ Inverse constructor for `create-link` — always undoes to `delete-link`.

use super::mutation::CreateLink;
use crate::artifacts::layout::mutations::{delete_link, LayoutMutation};
use crate::artifacts::layout::LayoutSnapshot;

//#region 🖇️CreateLink
pub async fn inverse_create_link(payload: &CreateLink, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteLink(delete_link::mutation::DeleteLink { id: payload.link.id.clone() })]
}
//#endregion 🖇️CreateLink
