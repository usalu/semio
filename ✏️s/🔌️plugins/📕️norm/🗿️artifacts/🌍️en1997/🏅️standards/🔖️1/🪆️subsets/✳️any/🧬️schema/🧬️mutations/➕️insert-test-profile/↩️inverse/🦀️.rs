use crate::insert_test_profile::InsertTestProfile;
use crate::remove_test_profile::RemoveTestProfile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertTestProfile, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_test_profile::RemoveTestProfile> {
    let at = base.piles.iter().find(|row| row.id == payload.pile_id).map_or(0, |parent| payload.index.min(parent.test_profiles.len()));
    Ok(vec![Op::from(RemoveTestProfile { pile_id: payload.pile_id.clone(), index: at })])
}
