//! ↩️ Inverse for `SavedCameras`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingSavedCamera, ShootingSavedCameraPatch, ShootingSnapshot};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &ShootingSnapshot, mutation: &CollectionMutation<String, ShootingSavedCamera, ShootingSavedCameraPatch>) -> Vec<ShootingMutation> {
    vec![ShootingMutation::SavedCameras(inverse_collection_mutation(&base.saved_cameras, mutation))]
}
//#endregion 🔖️Inverse
