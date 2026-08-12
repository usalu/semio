//! ↩️ `change-masonry-class` inverse — restores the pre-change `masonry_class` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_masonry_class::mutation::ChangeMasonryClass;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMasonryClass, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeMasonryClass(ChangeMasonryClass { new_masonry_class: base.masonry_class.clone() })]
}
//#endregion 🔖️Inverse
