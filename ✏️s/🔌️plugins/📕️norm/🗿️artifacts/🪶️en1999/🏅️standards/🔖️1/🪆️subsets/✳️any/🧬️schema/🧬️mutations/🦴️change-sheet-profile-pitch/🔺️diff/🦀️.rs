//! 🔺️ `change-sheet-profile-pitch` diff: patches `pitch` of the addressed connection row.

use crate::change_sheet_profile_pitch::ChangeSheetProfilePitch;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SheetProfilesPatch, En1999SheetProfilesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetProfilePitch, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_pitch.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "sheet-profile", Vec::<String>::new());
    };
    if r0.pitch == payload.new_pitch {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this hole stagger.");
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_profiles: Some(En1999SheetProfilesRows::modification(&payload.sheet_profile_id, En1999SheetProfilesPatch { pitch: Some(payload.new_pitch), ..Default::default() })), ..Default::default() })
}
