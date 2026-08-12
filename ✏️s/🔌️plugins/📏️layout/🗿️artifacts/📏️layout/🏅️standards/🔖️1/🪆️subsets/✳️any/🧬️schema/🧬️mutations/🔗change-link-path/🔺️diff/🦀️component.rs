//! 🔺 Diff constructor for `change-link-path`.

use super::mutation::ChangeLinkPath;
use crate::artifacts::layout::schema::diff::{LayoutLinkPatchEntry, LayoutLinksDelta};
use crate::artifacts::layout::{ImageLinkPatch, LayoutDiff, LayoutSnapshot};

//#region 🔗ChangeLinkPath
pub fn diff_change_link_path(payload: &ChangeLinkPath, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        links: Some(LayoutLinksDelta {
            patched: vec![LayoutLinkPatchEntry { id: payload.id.clone(), patch: ImageLinkPatch { path: Some(payload.new_path.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔗ChangeLinkPath
