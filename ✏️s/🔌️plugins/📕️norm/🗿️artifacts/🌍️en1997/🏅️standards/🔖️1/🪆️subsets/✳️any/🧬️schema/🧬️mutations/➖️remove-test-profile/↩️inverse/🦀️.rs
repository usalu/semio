use crate::remove_test_profile::RemoveTestProfile;
use crate::insert_test_profile::InsertTestProfile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveTestProfile, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_test_profile::InsertTestProfile> {
    Ok(match base.piles.iter().find(|row| row.id == payload.pile_id).and_then(|parent| parent.test_profiles.get(payload.index).cloned()) {
        Some(test_profile) => vec![Op::from(InsertTestProfile { pile_id: payload.pile_id.clone(), index: payload.index, test_profile })],
        None => Vec::new(),
    })
}
