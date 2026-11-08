//! 🔺️ `change-sheet-profile-sheet-id` diff: patches `sheet_id` of the addressed sheet_profile stiffener row.

use crate::change_sheet_profile_sheet_id::ChangeSheetProfileSheetId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SheetProfilesPatch, En1999SheetProfilesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetProfileSheetId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "sheet-profile", Vec::<String>::new());
    };
    if r0.sheet_id == payload.new_sheet_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The sheet_profile stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_profiles: Some(En1999SheetProfilesRows::modification(&payload.sheet_profile_id, En1999SheetProfilesPatch { sheet_id: Some(payload.new_sheet_id.clone()), ..Default::default() })), ..Default::default() })
}
