//! 🔺️ `change-masonry-class` sparse diff construction — writes only `En1996Diff.masonry_class` from the payload.

use crate::diff::En1996Diff;
use crate::mutations::change_masonry_class::ChangeMasonryClass;
use crate::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.masonry_class == payload.new_masonry_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Masonry class already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { masonry_class: Some(payload.new_masonry_class), ..Default::default() })
}
//#endregion 🔖️Diff
