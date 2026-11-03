use crate::remove_test_profile::RemoveTestProfile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows, En1997PilesTestProfilesRows};

pub fn diff(payload: &RemoveTestProfile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.piles.iter().find(|row| row.id == payload.pile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.pile_id), vec![payload.pile_id.clone()]);
    };
    if payload.index >= parent.test_profiles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("test-profile #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.pile_id, En1997PilesPatch { test_profiles: En1997PilesTestProfilesRows::removal(&parent.test_profiles, payload.index), ..Default::default() })), ..Default::default() })
}
