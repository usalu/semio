//! ↩️ Inverse for `SavedCameras`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingSavedCamera, ShootingSavedCameraPatch, ShootingFixture};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &ShootingFixture, mutation: &CollectionMutation<String, ShootingSavedCamera, ShootingSavedCameraPatch>) -> Vec<ShootingMutation> {
    vec![ShootingMutation::SavedCameras(inverse_collection_mutation(&base.saved_cameras, mutation))]
}
//#endregion 🔖️Inverse
