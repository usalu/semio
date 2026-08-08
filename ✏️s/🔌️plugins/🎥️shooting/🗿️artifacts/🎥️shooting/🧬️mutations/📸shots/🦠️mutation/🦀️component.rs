//! 📸 Shooting mutation — `Shots` apply.
use crate::artifacts::shooting::{ShootingShot, ShootingShotPatch, ShootingFixture};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
pub fn apply(fixture: &mut ShootingFixture, mutation: &CollectionMutation<String, ShootingShot, ShootingShotPatch>) {
    apply_collection_mutation(&mut fixture.shots, mutation);
}
//#endregion 🔖️Mutation
