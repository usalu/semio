//! 🔺️ `change-correction-as-of` — sparse diff construction.

use crate::change_correction_as_of::ChangeCorrectionAsOf;
use semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCorrectionAsOf, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.correction_as_of == payload.new_correction_as_of {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Correction as of already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { correction_as_of: Some(payload.new_correction_as_of), ..Default::default() })
}
//#endregion 🔖️Diff
