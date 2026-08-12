//! 🔺 Diff constructor for `delete-link`.

use super::mutation::DeleteLink;
use crate::artifacts::layout::schema::diff::LayoutLinksDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeleteLink
pub fn diff_delete_link(payload: &DeleteLink, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { links: Some(LayoutLinksDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeleteLink
