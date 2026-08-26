//! 🔺️ `change-mortar` sparse diff construction — writes only `En1996Diff.mortar` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_mortar::mutation::ChangeMortar;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMortar, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.mortar == payload.new_mortar {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mortar already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { mortar: Some(payload.new_mortar.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
