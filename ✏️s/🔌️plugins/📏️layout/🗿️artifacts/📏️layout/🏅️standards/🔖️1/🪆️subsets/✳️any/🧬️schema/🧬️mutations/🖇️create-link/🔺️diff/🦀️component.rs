//! 🔺 Diff constructor for `create-link`.

use super::mutation::CreateLink;
use crate::artifacts::layout::schema::diff::LayoutLinksDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🖇️CreateLink
pub async fn diff_create_link(payload: &CreateLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.links.iter().any(|link| link.id == payload.link.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A link with id \"{}\" already exists.", payload.link.id), [payload.link.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { added: vec![payload.link.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🖇️CreateLink
