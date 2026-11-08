//! 🔺️ `insert-sheet-profile` diff.

use crate::insert_sheet_profile::InsertSheetProfile;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SheetProfilesRows};

pub fn diff(payload: &InsertSheetProfile, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.sheet_profiles.iter().any(|existing| existing.id == payload.sheet_profile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Sheet profile id {} already exists.", payload.sheet_profile.id), [payload.sheet_profile.id.clone()]);
    }
    if (payload.index as usize) > base.sheet_profiles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.sheet_profiles.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_profiles: Some(En1999SheetProfilesRows::insertion(payload.index as usize, payload.sheet_profile.clone())), ..Default::default() })
}
