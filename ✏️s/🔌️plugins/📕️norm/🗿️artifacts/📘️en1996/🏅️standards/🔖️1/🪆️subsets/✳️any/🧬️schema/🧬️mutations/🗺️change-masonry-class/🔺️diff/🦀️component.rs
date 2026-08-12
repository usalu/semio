//! 🔺️ `change-masonry-class` sparse diff construction — writes only `En1996Diff.masonry_class` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_masonry_class::mutation::ChangeMasonryClass;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMasonryClass, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { masonry_class: Some(payload.new_masonry_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
