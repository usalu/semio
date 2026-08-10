//! 📸 Shooting mutation — `Shots` apply.
use crate::artifacts::shooting::{ShootingShot, ShootingShotPatch, ShootingSnapshot};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
pub fn apply(fixture: &mut ShootingSnapshot, mutation: &CollectionMutation<String, ShootingShot, ShootingShotPatch>) {
    apply_collection_mutation(&mut fixture.shots, mutation);
}
//#endregion 🔖️Mutation
