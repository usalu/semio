//! 🔺️ `change-deck-type` — sparse diff construction.

use super::mutation::ChangeDeckType;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDeckType, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.deck_type == payload.new_deck_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Deck type already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { deck_type: Some(payload.new_deck_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
