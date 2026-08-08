//! ↩️ Inverse for `Assets`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingFixture};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &ShootingFixture, mutation: &CollectionMutation<String, ShootingAsset, ShootingAssetPatch>) -> Vec<ShootingMutation> {
    vec![ShootingMutation::Assets(inverse_collection_mutation(&base.assets, mutation))]
}
//#endregion 🔖️Inverse
