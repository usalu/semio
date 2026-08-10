//! 📦 Shooting mutation — `Assets` apply.
use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingSnapshot};
use protocol::{apply_collection_mutation, CollectionMutation};

//#region 🔖️Mutation
pub fn apply(fixture: &mut ShootingSnapshot, mutation: &CollectionMutation<String, ShootingAsset, ShootingAssetPatch>) {
    apply_collection_mutation(&mut fixture.assets, mutation);
}
//#endregion 🔖️Mutation
