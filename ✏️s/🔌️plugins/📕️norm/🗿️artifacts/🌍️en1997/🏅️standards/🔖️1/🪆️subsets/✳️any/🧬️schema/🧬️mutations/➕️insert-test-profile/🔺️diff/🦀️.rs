use crate::insert_test_profile::InsertTestProfile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows, En1997PilesTestProfilesRows};

pub fn diff(payload: &InsertTestProfile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.piles.iter().find(|row| row.id == payload.pile_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.pile_id), vec![payload.pile_id.clone()]);
    };
    if parent.test_profiles.iter().any(|existing| existing.id == payload.test_profile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile test profile id {} already exists.", payload.test_profile.id), [payload.test_profile.id.clone()]);
    }
    if payload.index > parent.test_profiles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.test_profiles.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.pile_id, En1997PilesPatch { test_profiles: En1997PilesTestProfilesRows::insertion(payload.index, payload.test_profile.clone()), ..Default::default() })), ..Default::default() })
}
