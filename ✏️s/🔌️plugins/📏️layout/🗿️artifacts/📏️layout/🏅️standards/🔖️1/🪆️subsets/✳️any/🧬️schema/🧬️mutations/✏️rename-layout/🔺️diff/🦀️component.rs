//! 🔺 Diff constructor for `rename-layout` — builds `LayoutDiff` sparsely from the payload.

use super::mutation::RenameLayout;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region ✏️RenameLayout
pub fn diff_rename_layout(payload: &RenameLayout, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Layout already has that name.");
    }
    protocol::MutationOutcome::new(LayoutDiff { name: Some(payload.new_name.clone()), ..Default::default() })
}
//#endregion ✏️RenameLayout
