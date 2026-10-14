//! 🔺️ `change-annex` — sparse diff construction.

use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.annex == payload.new_annex {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Annex already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { annex: Some(payload.new_annex), ..Default::default() })
}
//#endregion 🔖️Diff
