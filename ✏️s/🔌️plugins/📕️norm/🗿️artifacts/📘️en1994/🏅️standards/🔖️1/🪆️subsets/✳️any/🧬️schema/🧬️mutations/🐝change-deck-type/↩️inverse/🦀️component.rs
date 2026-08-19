//! ↩️ `change-deck-type` — undo restores BASE's deck_type.

use super::mutation::ChangeDeckType;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDeckType, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeDeckType(ChangeDeckType { new_deck_type: base.deck_type.clone() })]
}
//#endregion 🔖️Inverse
