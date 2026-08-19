//! 🔺 Diff constructor for `delete-link`.

use super::mutation::DeleteLink;
use crate::artifacts::layout::schema::diff::LayoutLinksDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeleteLink
pub async fn diff_delete_link(payload: &DeleteLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.links.iter().any(|link| link.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteLink
