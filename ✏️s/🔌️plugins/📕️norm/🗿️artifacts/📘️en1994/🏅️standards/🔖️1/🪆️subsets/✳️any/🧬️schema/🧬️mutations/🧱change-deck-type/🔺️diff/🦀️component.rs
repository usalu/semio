//! 🔺️ `change-deck-type` — sparse diff construction.

use super::mutation::ChangeDeckType;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeckType, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { deck_type: Some(payload.new_deck_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
