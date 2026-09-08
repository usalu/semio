//! ↩️ `change-fire-rating` inverse — restores the pre-change `fire_rating` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_fire_rating::ChangeFireRating;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireRating, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: base.fire_rating })]
}
//#endregion 🔖️Inverse
