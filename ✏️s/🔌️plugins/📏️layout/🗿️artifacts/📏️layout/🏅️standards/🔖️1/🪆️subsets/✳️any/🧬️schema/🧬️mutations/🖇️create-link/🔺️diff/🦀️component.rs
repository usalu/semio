//! 🔺 Diff constructor for `create-link`.

use super::mutation::CreateLink;
use crate::artifacts::layout::schema::diff::LayoutLinksDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🖇️CreateLink
pub fn diff_create_link(payload: &CreateLink, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { links: Some(LayoutLinksDelta { added: vec![payload.link.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🖇️CreateLink
