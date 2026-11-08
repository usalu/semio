//! 🔺️ `remove-sheet-profile` diff.

use crate::remove_sheet_profile::RemoveSheetProfile;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SheetProfilesRows};

pub fn diff(payload: &RemoveSheetProfile, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.sheet_profiles.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown sheet profile id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { sheet_profiles: Some(En1999SheetProfilesRows::removal(&base.sheet_profiles, index)), ..Default::default() })
}
