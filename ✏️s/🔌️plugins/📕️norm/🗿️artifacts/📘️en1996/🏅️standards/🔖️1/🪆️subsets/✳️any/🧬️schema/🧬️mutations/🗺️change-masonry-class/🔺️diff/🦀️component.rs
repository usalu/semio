//! 🔺️ `change-masonry-class` sparse diff construction — writes only `En1996Diff.masonry_class` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_masonry_class::mutation::ChangeMasonryClass;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMasonryClass, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.masonry_class == payload.new_masonry_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Masonry class already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { masonry_class: Some(payload.new_masonry_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
