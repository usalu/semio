//! ↩️ `change-pile-n-profiles` inverse — restores the pre-change `pile_n_profiles` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_pile_n_profiles::mutation::ChangePileNProfiles;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangePileNProfiles, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangePileNProfiles(ChangePileNProfiles { new_pile_n_profiles: base.pile_n_profiles.clone() })]
}
//#endregion 🔖️Inverse
