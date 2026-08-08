//! 🎥 Shooting mutation — `SavedCameras` apply.
use crate::artifacts::shooting::{ShootingSavedCamera, ShootingSavedCameraPatch, ShootingSnapshot};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
pub fn apply(fixture: &mut ShootingSnapshot, mutation: &CollectionMutation<String, ShootingSavedCamera, ShootingSavedCameraPatch>) {
    apply_collection_mutation(&mut fixture.saved_cameras, mutation);
}
//#endregion 🔖️Mutation
