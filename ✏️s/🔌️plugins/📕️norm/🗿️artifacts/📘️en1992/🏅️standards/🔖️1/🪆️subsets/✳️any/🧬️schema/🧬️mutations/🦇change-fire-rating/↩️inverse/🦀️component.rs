//! ↩️ `change-fire-rating` inverse — restores the pre-change `fire_rating` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_fire_rating::mutation::ChangeFireRating;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireRating, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: base.fire_rating.clone() })]
}
//#endregion 🔖️Inverse
