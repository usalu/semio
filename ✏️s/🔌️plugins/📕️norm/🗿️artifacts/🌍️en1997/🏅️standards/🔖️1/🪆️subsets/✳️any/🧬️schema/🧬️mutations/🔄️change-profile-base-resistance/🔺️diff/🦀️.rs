//! 🔺️ `change-profile-base-resistance` diff: patches `base_resistance` of the addressed pile test profile row.

use crate::change_profile_base_resistance::ChangeProfileBaseResistance;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows, En1997PilesTestProfilesPatch, En1997PilesTestProfilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeProfileBaseResistance, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_base_resistance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.pile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.pile_id), vec![payload.pile_id.clone()]);
    };
    let Some(r1) = r0.test_profiles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("profile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.base_resistance == payload.new_base_resistance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile test profile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.pile_id, En1997PilesPatch { test_profiles: En1997PilesTestProfilesRows::modification(&payload.id, En1997PilesTestProfilesPatch { base_resistance: Some(payload.new_base_resistance), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
