//! 🔺️ `change-tightness-class` sparse diff construction — writes only `En1992Diff.tightness_class` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_tightness_class::mutation::ChangeTightnessClass;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeTightnessClass, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.tightness_class == payload.new_tightness_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Tightness class already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { tightness_class: Some(payload.new_tightness_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
