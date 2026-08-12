//! 🔺 Diff constructor for `rename-layout` — builds `LayoutDiff` sparsely from the payload.

use super::mutation::RenameLayout;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region ✏️RenameLayout
pub fn diff_rename_layout(payload: &RenameLayout, _base: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { name: Some(payload.new_name.clone()), ..Default::default() }
}
//#endregion ✏️RenameLayout
