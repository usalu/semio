//! 🔺️ `change-sheet-profile-bottom-flange` diff: patches `bottom_flange` of the addressed connection row.

use crate::change_sheet_profile_bottom_flange::ChangeSheetProfileBottomFlange;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SheetProfilesPatch, En1999SheetProfilesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetProfileBottomFlange, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_bottom_flange.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.sheet_profiles.iter().find(|row| row.id == payload.sheet_profile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "sheet-profile", Vec::<String>::new());
    };
    if r0.bottom_flange == payload.new_bottom_flange {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this hole stagger.");
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_profiles: Some(En1999SheetProfilesRows::modification(&payload.sheet_profile_id, En1999SheetProfilesPatch { bottom_flange: Some(payload.new_bottom_flange), ..Default::default() })), ..Default::default() })
}
