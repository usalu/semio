//! 🔺️ `change-correction-as-of` — sparse diff construction.

use super::mutation::ChangeCorrectionAsOf;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeCorrectionAsOf, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.correction_as_of == payload.new_correction_as_of {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Correction as of already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { correction_as_of: Some(payload.new_correction_as_of.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
